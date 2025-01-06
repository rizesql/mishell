use std::path::PathBuf;

use crate::{exec, Error};

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
        let lexer = ast::tokenizer_v2::Lexer::new(command.chars());
        let tokens = lexer.collect::<Vec<_>>();

        let mut parser = ast::parser_v2::Parser::new(tokens.as_slice());
        tracing::info!("{:?}", tokens);

        if let Some(p) = parser.program() {
            tracing::info!("PROGRAM: \n{p}  \n{:?}", p);
        }

        tracing::info!("left {:?}", parser.remaining());
        Ok(exec::ExitCode::success())
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
