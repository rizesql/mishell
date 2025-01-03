use crate::ast;

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
