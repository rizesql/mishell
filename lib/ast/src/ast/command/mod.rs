use crate::{ast, one_of, parser_v2::Parser};

mod arithmetic;
pub mod compound;
mod simple;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Simple(simple::SimpleCommand),
    Compound(compound::CompoundCommand, Option<ast::Redirects>),
    Function(),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Simple(cmd) => write!(f, "{cmd}"),
            Command::Compound(cmd, redirects) => {
                write!(f, "{cmd}")?;

                if let Some(redirects) = redirects {
                    write!(f, "{redirects}")?;
                }

                Ok(())
            }
            Command::Function() => todo!(),
        }
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn command(&mut self) -> Option<Command> {
        self.transaction(|parser| {
            let res = one_of!(
                parser,
                || parser.simple_command().map(Command::Simple),
                || {
                    let cmd = parser.compound_command()?;
                    let redirects = parser.redirects();
                    Some(Command::Compound(cmd, redirects))
                }
            )?;

            Some(res)
        })
    }
}
