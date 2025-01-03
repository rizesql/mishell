use crate::INDENT;

use super::CompoundBlock;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoScope(CompoundBlock);

impl std::fmt::Display for DoScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "do")?;
        writeln!(indenter::indented(f).with_str(INDENT), "{}", self.0)?;
        writeln!(f)?;
        write!(f, "done")
    }
}
