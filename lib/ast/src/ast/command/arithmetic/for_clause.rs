use crate::{
    ast::command::compound::DoScope,
    parser_v2::Parser,
    tokens::{Paren, Symbol},
};

use super::UnexpandedExpr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct For {
    init: Option<UnexpandedExpr>,
    cond: Option<UnexpandedExpr>,
    post: Option<UnexpandedExpr>,
    scope: DoScope,
}

impl std::fmt::Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "for ((")?;

        if let Some(init) = &self.init {
            write!(f, "{init}")?;
        }

        write!(f, "; ")?;

        if let Some(cond) = &self.cond {
            write!(f, "{cond}")?;
        }

        write!(f, "; ")?;

        if let Some(post) = &self.post {
            write!(f, "{post}")?;
        }

        write!(f, "))")?;

        write!(f, "{}", self.scope)
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn arithmetic_for_clause(&mut self) -> Option<For> {
        self.transaction(|parser| {
            parser.consume("for")?;
            parser.consume(Paren::open())?;
            parser.consume(Paren::open())?;

            let init = parser.arithmetic_expr();
            parser.consume(Symbol::Semicolon)?;
            let cond = parser.arithmetic_expr();
            parser.consume(Symbol::Semicolon)?;
            let post = parser.arithmetic_expr();

            parser.consume(Paren::close())?;
            parser.consume(Paren::close())?;

            parser.seq_separator()?;
            let scope = parser.do_scope()?;

            Some(For {
                init,
                cond,
                post,
                scope,
            })
        })
    }
}
