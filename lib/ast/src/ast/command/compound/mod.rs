mod do_scope;
mod for_clause;
mod if_clause;
mod scope;
mod subshell;
mod until_clause;
mod while_clause;

pub use arithmetic::{Arithmetic, For as ArithmeticFor};
pub(crate) use do_scope::DoScope;
pub use for_clause::For;
pub use if_clause::If;
pub use scope::Scope;
pub use subshell::Subshell;
pub use until_clause::Until;
pub use while_clause::While;

use crate::ast::command::arithmetic;
use crate::ast::PipelineSeq;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompoundCommand {
    Arithmetic(arithmetic::Arithmetic),
    ArithmeticFor(arithmetic::For),
    Scope(scope::Scope),
    Subshell(subshell::Subshell),
    If(if_clause::If),
    For(for_clause::For),
    While(while_clause::While),
    Until(until_clause::Until),
}

impl std::fmt::Display for CompoundCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompoundCommand::Arithmetic(cmd) => write!(f, "{cmd}"),
            CompoundCommand::ArithmeticFor(cmd) => write!(f, "{cmd}"),
            CompoundCommand::Scope(cmd) => write!(f, "{cmd}"),
            CompoundCommand::Subshell(cmd) => write!(f, "{cmd}"),
            CompoundCommand::If(cmd) => write!(f, "{cmd}"),
            CompoundCommand::For(cmd) => write!(f, "{cmd}"),
            CompoundCommand::While(cmd) => write!(f, "{cmd}"),
            CompoundCommand::Until(cmd) => write!(f, "{cmd}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CompoundBlock(Vec<CompoundBlockItem>);

impl std::fmt::Display for CompoundBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, it) in self.0.iter().enumerate() {
            if idx > 0 {
                writeln!(f)?;
            }

            write!(f, "{}", it.seq)?;

            if idx != self.0.len() - 1 || !matches!(it.separator, Separator::Seq) {
                write!(f, "{}", it.separator)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CompoundBlockItem {
    seq: PipelineSeq,
    separator: Separator,
}

impl std::fmt::Display for CompoundBlockItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.seq)?;
        write!(f, "{}", self.separator)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Separator {
    /// The prev command runs asynchronously (`&`)
    Async,
    /// The prev command runs synchronously (`;`)
    Seq,
}

impl std::fmt::Display for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Separator::Async => write!(f, "&"),
            Separator::Seq => write!(f, ";"),
        }
    }
}
