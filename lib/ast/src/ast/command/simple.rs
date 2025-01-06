use crate::{ast, min_once, one_of, parser_v2::Parser, tokens::Symbol};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleCommand {
    prefix: Option<Affixes>,
    word: Option<ast::Word>,
    suffix: Option<Affixes>,
}

impl std::fmt::Display for SimpleCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut wrote = false;

        if let Some(prefix) = &self.prefix {
            write!(f, "{prefix}")?;
            wrote = true;
        }

        if let Some(word) = &self.word {
            let space = if wrote { " " } else { "" };
            write!(f, "{space}{word}")?;
            wrote = true;
        }

        if let Some(suffix) = &self.suffix {
            let space = if wrote { " " } else { "" };
            write!(f, "{space}{suffix}")?;
        }

        Ok(())
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn simple_command(&mut self) -> Option<SimpleCommand> {
        self.transaction(|parser| {
            let res = one_of!(
                parser,
                || {
                    let prefix = parser.prefixes()?;
                    let word_suffix = parser.word_suffix();

                    match word_suffix {
                        Some((word, suffix)) => Some(SimpleCommand {
                            prefix: Some(prefix),
                            word: Some(word),
                            suffix,
                        }),
                        None => Some(SimpleCommand {
                            prefix: Some(prefix),
                            word: None,
                            suffix: None,
                        }),
                    }
                },
                || {
                    let word = parser.non_reserved_word()?;
                    let suffix = parser.suffixes();

                    Some(SimpleCommand {
                        prefix: None,
                        word: Some(word),
                        suffix,
                    })
                }
            );
            res
        })
    }

    fn word_suffix(&mut self) -> Option<(ast::Word, Option<Affixes>)> {
        self.transaction(|parser| {
            let word = parser.non_reserved_word()?;
            let suffix = parser.suffixes();

            Some((word, suffix))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Affix {
    Redirect(ast::Redirect),
    Word(ast::Word),
    Assignment(Assignment),
}

impl std::fmt::Display for Affix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Affix::Redirect(target) => write!(f, "{target}"),
            Affix::Word(target) => write!(f, "{target}"),
            Affix::Assignment(assignment) => write!(f, "{}", assignment.rhs),
        }
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    fn prefixes(&mut self) -> Option<Affixes> {
        self.transaction(|parser| {
            let res = min_once!(|| parser.prefix()).map(Affixes)?;
            Some(res)
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn prefix(&mut self) -> Option<Affix> {
        self.transaction(|parser| {
            let res = one_of!(parser, || parser.redirect().map(Affix::Redirect), || parser
                .assignment()
                .map(Affix::Assignment))?;

            Some(res)
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn suffixes(&mut self) -> Option<Affixes> {
        self.transaction(|parser| {
            let res = min_once!(|| parser.suffix()).map(Affixes)?;
            Some(res)
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn suffix(&mut self) -> Option<Affix> {
        self.transaction(|parser| {
            let res = one_of!(
                parser,
                || parser.redirect().map(Affix::Redirect),
                || parser.assignment().map(Affix::Assignment),
                || parser.word().map(Affix::Word)
            )?;

            Some(res)
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn affix(&mut self) -> Option<Affix> {
        self.transaction(|parser| {
            let res = one_of!(
                parser,
                || parser.redirect().map(Affix::Redirect),
                || parser.word().map(Affix::Word),
                || parser.assignment().map(Affix::Assignment)
            );
            res
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Affixes(Vec<Affix>);

impl std::fmt::Display for Affixes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();

        if let Some(fst) = iter.next() {
            write!(f, "{fst}")?;

            for affix in iter {
                write!(f, " {affix}")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    lhs: String,
    rhs: ast::Word,
    append: bool,
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = if self.append { "+=" } else { "=" };
        write!(f, "{}{}{}", self.lhs, op, self.rhs)
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn assignment(&mut self) -> Option<Assignment> {
        self.transaction(|parser| {
            let lhs = parser.var()?;

            let append = parser
                .choice(|tok| tok.as_symbol().filter(|s| *s == Symbol::Plus))
                .is_some();
            parser.consume(Symbol::Eq)?;

            let rhs = parser.word()?;

            let res = Assignment { lhs, rhs, append };
            Some(res)
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn var(&mut self) -> Option<String> {
        self.transaction(|parser| {
            let w = parser.choice(|tok| tok.as_word().filter(|w| !w.is_empty()))?;

            let (fst, rest) = {
                let mut iter = w.chars();
                let fst = iter.next().unwrap();
                let rest = iter.as_str();
                (fst, rest)
            };

            if !matches!(fst, '_' | 'a'..='z' | 'A'..='Z') {
                return None;
            };

            if !rest.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return None;
            }

            Some(w)
        })
    }
}
