use crate::{ast::command, parser_v2::Parser};

use super::CompoundBlock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Until {
    cond: CompoundBlock,
    scope: command::compound::DoScope,
}

impl std::fmt::Display for Until {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "until {}; {}", self.cond, self.scope)
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn until_clause(&mut self) -> Option<Until> {
        self.transaction(|parser| {
            parser.consume("until")?;
            let cond = parser.compound_block()?;
            let scope = parser.do_scope()?;

            Some(Until { cond, scope })
        })
    }
}
