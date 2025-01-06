use crate::{
    ast, min_once, one_of,
    parser_v2::Parser,
    tokens::{Symbol, Token},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct For {
    lhs: String,
    rhs: Option<Vec<ast::Word>>,
    scope: ast::command::compound::DoScope,
}

impl std::fmt::Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "for {} in ", self.lhs)?;

        if let Some(rhs) = &self.rhs {
            let mut iter = rhs.iter();

            if let Some(fst) = iter.next() {
                write!(f, "{fst}")?;

                for it in iter {
                    write!(f, " {it}")?;
                }
            }
        }

        writeln!(f, ";")?;
        write!(f, "{}", self.scope)
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn for_clause(&mut self) -> Option<For> {
        self.transaction(|parser| {
            parser.consume("for")?;

            let lhs = parser.name()?;

            parser.linebreak()?;
            parser.consume("in")?;
            let rhs = parser.words();
            tracing::info!("=================== REMAINING {:?}", rhs);

            parser.seq_separator()?;

            let scope = parser.do_scope()?;

            Some(For { lhs, rhs, scope })
        })
    }

    #[tracing::instrument(skip(self), ret)]
    pub fn seq_separator(&mut self) -> Option<()> {
        self.transaction(|parser| {
            one_of!(
                parser,
                || {
                    parser.consume(Symbol::Semicolon)?;
                    parser.linebreak()?;
                    Some(())
                },
                || parser.required_linebreak()
            )
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn words(&mut self) -> Option<Vec<ast::Word>> {
        self.transaction(|parser| {
            let res = min_once!(|| parser.non_reserved_word().map(ast::Word::from))?;
            Some(res)
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn name(&mut self) -> Option<String> {
        let res = self.choice(|tok| match tok {
            Token::Word(w) => Some(w.into()),
            _ => None,
        })?;
        Some(res)
    }
}
