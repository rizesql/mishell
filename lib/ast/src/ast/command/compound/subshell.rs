use crate::{parser_v2::Parser, tokens::Paren};

use super::CompoundBlock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subshell(CompoundBlock);

impl std::fmt::Display for Subshell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} )", self.0)
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn subshell(&mut self) -> Option<Subshell> {
        self.transaction(|parser| {
            parser.consume(Paren::open())?;
            let cmd = parser.compound_block()?;
            parser.consume(Paren::close())?;

            Some(Subshell(cmd))
        })
    }
}
