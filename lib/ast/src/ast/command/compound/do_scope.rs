use crate::{parser_v2::Parser, INDENT};

use super::CompoundBlock;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoScope(CompoundBlock);

impl std::fmt::Display for DoScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "do")?;
        writeln!(indenter::indented(f).with_str(INDENT), "{}", self.0)?;
        write!(f, "done")
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn do_scope(&mut self) -> Option<DoScope> {
        self.transaction(|parser| {
            parser.consume("do")?;
            let cmd = parser.compound_block()?;
            tracing::info!("=====================\n COMMAND {cmd}");
            tracing::info!("{:?}", parser.remaining());
            parser.consume("done")?;

            Some(DoScope(cmd))
        })
    }
}
