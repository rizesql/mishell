use crate::{parser_v2::Parser, tokens::Paren, INDENT};

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

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn scope(&mut self) -> Option<Scope> {
        self.transaction(|parser| {
            parser.consume(Paren::curly_open())?;
            let cmd = parser.compound_block()?;
            parser.consume(Paren::curly_close())?;

            Some(Scope(cmd))
        })
    }
}
