use crate::ast::command;

use super::CompoundBlock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct While {
    cond: CompoundBlock,
    body: command::compound::DoScope,
}

impl std::fmt::Display for While {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "while {}; {}", self.cond, self.body)
    }
}
