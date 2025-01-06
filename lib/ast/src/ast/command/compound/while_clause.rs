use crate::{ast::command, parser_v2::Parser};

use super::CompoundBlock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct While {
    cond: CompoundBlock,
    scope: command::compound::DoScope,
}

impl std::fmt::Display for While {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "while {}; {}", self.cond, self.scope)
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn while_clause(&mut self) -> Option<While> {
        self.transaction(|parser| {
            parser.consume("while")?;
            let cond = parser.compound_block()?;
            let scope = parser.do_scope()?;

            Some(While { cond, scope })
        })
    }
}
