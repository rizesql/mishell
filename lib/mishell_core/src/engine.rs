use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::{exec, Error};
use ast::parser::{self, ASTNode};
use ast::tokenizer::{debug_tokens, tokenizer};

#[derive(Debug, Clone)]
pub struct Engine {
    pub working_dir: PathBuf,
    last_exit_status: u8,
}

impl Engine {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            working_dir: std::env::current_dir()?,
            last_exit_status: 0,
        })
    }

    pub fn last_exit_status(&self) -> u8 {
        self.last_exit_status
    }

    pub fn set_last_exit_status(&mut self, exit_status: u8) {
        self.last_exit_status = exit_status;
    }

    pub async fn run(&mut self, command: String) -> Result<exec::ExitCode, Error> {
        let tokens = tokenizer(command).await;
        debug_tokens(&tokens);

        let mut parser = parser::Parser::new(tokens);

        let ast = parser.parse().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse command")
        })?;

        println!("{:#?}", ast);

        let _ = self.execute(&ast);

        Ok(exec::ExitCode::success())
    }

    pub fn execute(&mut self, ast: &ASTNode) -> Result<(), String> {
        match ast {
            ASTNode::Command { name, args } => self.execute_command(name, args),
            ASTNode::Sequence(nodes) => {
                for node in nodes {
                    self.execute(node)?;
                }
                Ok(())
            }
            ASTNode::Pipeline { left, right } => {
                let left_output = self.capture_output(left)?;
                self.execute_with_input(right, left_output)
            }
            ASTNode::Redirection {
                command,
                file,
                direction,
            } => {
                let mut cmd = Command::new("sh");
                cmd.arg("-c").arg(self.to_shell_command(command)?);

                match direction.as_str() {
                    ">" => {
                        cmd.stdout(Stdio::from(
                            std::fs::File::create(file).map_err(|e| e.to_string())?,
                        ));
                    }
                    "<" => {
                        cmd.stdin(Stdio::from(
                            std::fs::File::open(file).map_err(|e| e.to_string())?,
                        ));
                    }
                    _ => return Err(format!("Unknown redirection direction: {}", direction)),
                }

                cmd.status().map_err(|e| e.to_string())?;
                Ok(())
            }
            ASTNode::Logical {
                left,
                right,
                operator,
            } => {
                let left_result = self.execute(left);

                match operator.as_str() {
                    "&&" => {
                        if left_result.is_ok() {
                            self.execute(right)
                        } else {
                            left_result
                        }
                    }
                    "||" => {
                        if left_result.is_ok() {
                            Ok(())
                        } else {
                            self.execute(right)
                        }
                    }
                    _ => Err(format!("Unknown logical operator: {}", operator)),
                }
            }
            ASTNode::ForLoop {
                variable,
                values,
                body,
            } => {
                for value in values {
                    std::env::set_var(variable, value);
                    self.execute(body)?;
                }
                Ok(())
            }
            ASTNode::IfCondition {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.execute(condition).is_ok() {
                    self.execute(then_branch)
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)
                } else {
                    Ok(())
                }
            }
        }
    }

    fn execute_command(&mut self, name: &str, args: &[String]) -> Result<(), String> {
        if name == "exec" {
            if args.is_empty() {
                return Err("No command provided for 'exec'".to_string());
            }

            let mut cmd = Command::new(&args[0]);
            cmd.args(&args[1..]);

            let mut child = cmd
                .spawn()
                .map_err(|e| format!("Failed to execute command '{}': {}", args[0], e))?;

            return child
                .wait()
                .map(|status| {
                    if status.success() {
                        Ok(())
                    } else {
                        Err(format!(
                            "Command '{}' exited with status: {}",
                            args[0], status
                        ))
                    }
                })
                .unwrap_or_else(|e| {
                    Err(format!("Failed to wait for command '{}': {}", args[0], e))
                });
        } else if name == "cd" {
            if args.len() != 1 {
                return Err("cd requires exactly one argument".to_string());
            }
            let path = &args[0];
            std::env::set_current_dir(path)
                .map_err(|e| format!("Failed to change directory: {}", e))?;
            self.working_dir = std::env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?;
            return Ok(());
        }

        let mut cmd = Command::new(name);
        cmd.args(args);
        cmd.status()
            .map_err(|e| format!("Failed to execute command '{}': {}", name, e))
            .map(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(format!("Command '{}' exited with status: {}", name, status))
                }
            })
            .unwrap_or_else(|e| Err(format!("Failed to execute '{}': {}", name, e)))
    }

    fn capture_output(&mut self, ast: &ASTNode) -> Result<Vec<u8>, String> {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(self.to_shell_command(ast)?);
        let output = cmd.output().map_err(|e| e.to_string())?;
        Ok(output.stdout)
    }

    fn execute_with_input(&mut self, ast: &ASTNode, input: Vec<u8>) -> Result<(), String> {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(self.to_shell_command(ast)?);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::inherit());
        let mut child = cmd.spawn().map_err(|e| e.to_string())?;
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(&input).map_err(|e| e.to_string())?;
        }
        child.wait().map_err(|e| e.to_string())?;
        Ok(())
    }

    fn to_shell_command(&self, ast: &ASTNode) -> Result<String, String> {
        match ast {
            ASTNode::Command { name, args } => {
                let args_str = args.join(" ");
                Ok(format!("{} {}", name, args_str))
            }
            ASTNode::Pipeline { left, right } => {
                let left_cmd = self.to_shell_command(left)?;
                let right_cmd = self.to_shell_command(right)?;
                Ok(format!("{} | {}", left_cmd, right_cmd))
            }
            ASTNode::Redirection {
                command,
                file,
                direction,
            } => {
                let cmd_str = self.to_shell_command(command)?;
                let redirection = match direction.as_str() {
                    ">" => format!("{} > {}", cmd_str, file),
                    "<" => format!("{} < {}", cmd_str, file),
                    _ => return Err(format!("Unknown redirection direction: {}", direction)),
                };
                Ok(redirection)
            }
            _ => Err("Unsupported AST node type for shell command".to_string()),
        }
    }
}

impl AsRef<Engine> for Engine {
    fn as_ref(&self) -> &Engine {
        self
    }
}

impl AsMut<Engine> for Engine {
    fn as_mut(&mut self) -> &mut Engine {
        self
    }
}
