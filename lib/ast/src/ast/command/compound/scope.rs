use crate::INDENT;

use super::CompoundBlock;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope(CompoundBlock);

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{ ")?;
        write!(indenter::indented(f).with_str(INDENT), "{}", self.0)?;
        writeln!(f)?;
        write!(f, "}}")?;
        Ok(())
    }
}
