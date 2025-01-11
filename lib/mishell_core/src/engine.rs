use std::io::Write;
use std::path::PathBuf;

use ast::tokenizer::{debug_tokens, tokenizer};

use ast::parser::{self, ASTNode};

use crate::{exec, Error};

use std::process::{Command, Stdio};

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

    pub fn run(&mut self, command: String) -> Result<exec::ExitCode, Error> {
        // let lexer = ast::tokenizer_v2::Lexer::new(command.chars());
        let tokens = tokenizer(command);
        // let tokens = lexer.collect::<Vec<_>>();

        debug_tokens(&tokens);

        let mut parser = parser::Parser::new(tokens);

        let ast = parser.parse().expect("Failed to parse nigger");



        println!("{:#?}", ast);


        let _ = Engine::execute(&ast);

        // let mut parser = ast::parser_v2::Parser::new(tokens.as_slice());
        // tracing::info!("{:?}", tokens);

       
        Ok(exec::ExitCode::success())
    }

    pub fn execute(ast: &ASTNode) -> Result<(), String> {
        match ast {
            ASTNode::Command { name, args } => Self::execute_command(name, args),
            ASTNode::Sequence(nodes) => {
                for node in nodes {
                    Self::execute(node)?;
                }
                Ok(())
            }
            ASTNode::Pipeline { left, right } => {
                let left_output = Self::capture_output(left)?;
                Self::execute_with_input(right, left_output)
            }
            ASTNode::Redirection { command, file, direction } => {
                let mut cmd = Command::new("sh");
                cmd.arg("-c").arg(Self::to_shell_command(command)?);

                match direction.as_str() {
                    ">" => {
                        cmd.stdout(Stdio::from(std::fs::File::create(file).map_err(|e| e.to_string())?));
                    }
                    "<" => {
                        cmd.stdin(Stdio::from(std::fs::File::open(file).map_err(|e| e.to_string())?));
                    }
                    _ => return Err(format!("Unknown redirection direction: {}", direction)),
                }

                cmd.status().map_err(|e| e.to_string())?;
                Ok(())
            }
            ASTNode::Logical { left, right, operator } => {
                let left_result = Self::execute(left);

                match operator.as_str() {
                    "&&" => {
                        if left_result.is_ok() {
                            Self::execute(right)
                        } else {
                            left_result
                        }
                    }
                    "||" => {
                        if left_result.is_ok() {
                            Ok(())
                        } else {
                            Self::execute(right)
                        }
                    }
                    _ => Err(format!("Unknown logical operator: {}", operator)),
                }
            }
            ASTNode::ForLoop { variable, values, body } => {
                for value in values {
                    std::env::set_var(variable, value);
                    Self::execute(body)?;
                }
                Ok(())
            }
            ASTNode::IfCondition { condition, then_branch, else_branch } => {
                if Self::execute(condition).is_ok() {
                    Self::execute(then_branch)
                } else if let Some(else_branch) = else_branch {
                    Self::execute(else_branch)
                } else {
                    Ok(())
                }
            }
        }
    }

    fn execute_command(name: &str, args: &[String]) -> Result<(), String> {
        let mut cmd = Command::new(name);
        cmd.args(args);
        cmd.status()
            .map_err(|e| format!("Failed to execute command '{}': {}", name, e))
            .map(|_| ())
    }

    fn capture_output(ast: &ASTNode) -> Result<Vec<u8>, String> {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(Self::to_shell_command(ast)?);
        let output = cmd.output().map_err(|e| e.to_string())?;
        Ok(output.stdout)
    }

    fn execute_with_input(ast: &ASTNode, input: Vec<u8>) -> Result<(), String> {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(Self::to_shell_command(ast)?);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::inherit());
        let mut child = cmd.spawn().map_err(|e| e.to_string())?;
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(&input).map_err(|e| e.to_string())?;
        }
        child.wait().map_err(|e| e.to_string())?;
        Ok(())
    }

    fn to_shell_command(ast: &ASTNode) -> Result<String, String> {
        match ast {
            ASTNode::Command { name, args } => {
                let args_str = args.join(" ");
                Ok(format!("{} {}", name, args_str))
            }
            _ => Err("Only command nodes can be converted to shell commands".to_string()),
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
