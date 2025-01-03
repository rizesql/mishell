use crate::ast::command;

use super::CompoundBlock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Until {
    cond: CompoundBlock,
    body: command::compound::DoScope,
}

impl std::fmt::Display for Until {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "until {}; {}", self.cond, self.body)
    }
}
