use crate::{min_once, one_of, parser_v2::Parser, INDENT};

use super::CompoundBlock;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If {
    cond: CompoundBlock,
    then: CompoundBlock,
    branches: Option<Vec<Else>>,
}

impl std::fmt::Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "if {}; then", self.cond)?;
        write!(indenter::indented(f).with_str(INDENT), "{}", self.then)?;

        if let Some(branches) = &self.branches {
            for branch in branches {
                write!(f, "{branch}")?;
            }
        }

        writeln!(f)?;
        write!(f, "fi")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Else {
    cond: Option<CompoundBlock>,
    block: CompoundBlock,
}

impl std::fmt::Display for Else {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        if let Some(cond) = &self.cond {
            writeln!(f, "elif {}; then", cond)?;
        } else {
            writeln!(f, "else")?;
        }

        write!(indenter::indented(f).with_str(INDENT), "{}", self.block)
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn if_clause(&mut self) -> Option<If> {
        self.transaction(|parser| {
            parser.consume("if")?;
            let cond = parser.compound_block()?;
            parser.consume("then")?;
            let then = parser.compound_block()?;
            let branches = parser.else_clauses();
            parser.consume("fi")?;

            Some(If {
                cond,
                then,
                branches,
            })
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn else_clauses(&mut self) -> Option<Vec<Else>> {
        self.transaction(|parser| {
            one_of!(
                parser,
                || {
                    let mut branches = min_once!(|| parser.elif_clause())?;
                    if let Some(branch) = parser.else_clause() {
                        branches.push(branch);
                    }
                    Some(branches)
                },
                || {
                    let branch = parser.else_clause()?;
                    Some(vec![branch])
                }
            )
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn elif_clause(&mut self) -> Option<Else> {
        self.transaction(|parser| {
            parser.consume("elif")?;
            let cond = parser.compound_block()?;
            parser.consume("then")?;
            let block = parser.compound_block()?;

            Some(Else {
                cond: Some(cond),
                block,
            })
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn else_clause(&mut self) -> Option<Else> {
        self.transaction(|parser| {
            parser.consume("else")?;
            let block = parser.compound_block()?;

            Some(Else { cond: None, block })
        })
    }
}
