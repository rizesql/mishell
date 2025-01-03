use crate::INDENT;

use super::CompoundBlock;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If {
    condition: CompoundBlock,
    then: CompoundBlock,
    branches: Option<Vec<Else>>,
}

impl std::fmt::Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "if {}; then", self.condition)?;
        write!(indenter::indented(f).with_str(INDENT), "{}", self.then)?;

        if let Some(branches) = &self.branches {
            for branch in branches {
                write!(f, "{branch}")?;
            }
        }

        writeln!(f)?;
        write!(f, "fi")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Else {
    cond: Option<CompoundBlock>,
    block: CompoundBlock,
}

impl std::fmt::Display for Else {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        if let Some(cond) = &self.cond {
            writeln!(f, "elif {}; then", cond)?;
        } else {
            writeln!(f, "else")?;
        }

        write!(indenter::indented(f).with_str(INDENT), "{}", self.block)
    }
}
