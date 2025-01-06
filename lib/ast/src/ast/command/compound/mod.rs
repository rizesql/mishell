mod do_scope;
mod for_clause;
mod if_clause;
mod scope;
mod subshell;
mod until_clause;
mod while_clause;

pub use arithmetic::{Arithmetic, For as ArithmeticFor};
pub(crate) use do_scope::DoScope;
pub use for_clause::For;
pub use if_clause::If;
pub use scope::Scope;
pub use subshell::Subshell;
pub use until_clause::Until;
pub use while_clause::While;

use crate::ast::command::arithmetic;
use crate::ast::PipelineSeq;
use crate::parser_v2::Parser;
use crate::tokens::Symbol;
use crate::{one_of, repeat};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompoundCommand {
    Arithmetic(arithmetic::Arithmetic),
    ArithmeticFor(arithmetic::For),
    Scope(scope::Scope),
    Subshell(subshell::Subshell),
    If(if_clause::If),
    For(for_clause::For),
    While(while_clause::While),
    Until(until_clause::Until),
}

impl std::fmt::Display for CompoundCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompoundCommand::Arithmetic(cmd) => write!(f, "{cmd}"),
            CompoundCommand::ArithmeticFor(cmd) => write!(f, "{cmd}"),
            CompoundCommand::Scope(cmd) => write!(f, "{cmd}"),
            CompoundCommand::Subshell(cmd) => write!(f, "{cmd}"),
            CompoundCommand::If(cmd) => write!(f, "{cmd}"),
            CompoundCommand::For(cmd) => write!(f, "{cmd}"),
            CompoundCommand::While(cmd) => write!(f, "{cmd}"),
            CompoundCommand::Until(cmd) => write!(f, "{cmd}"),
        }
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn compound_command(&mut self) -> Option<CompoundCommand> {
        self.transaction(|parser| {
            one_of!(
                parser,
                || parser.arithmetic().map(CompoundCommand::Arithmetic),
                || parser.scope().map(CompoundCommand::Scope),
                || parser.subshell().map(CompoundCommand::Subshell),
                || parser.for_clause().map(CompoundCommand::For),
                || parser.if_clause().map(CompoundCommand::If),
                || parser.while_clause().map(CompoundCommand::While),
                || parser.until_clause().map(CompoundCommand::Until),
                || parser
                    .arithmetic_for_clause()
                    .map(CompoundCommand::ArithmeticFor)
            )
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundBlock(Vec<CompoundBlockItem>);

impl CompoundBlock {
    pub fn new(inner: Vec<CompoundBlockItem>) -> Self {
        Self(inner)
    }
}

impl std::fmt::Display for CompoundBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, it) in self.0.iter().enumerate() {
            if idx > 0 {
                writeln!(f)?;
            }

            write!(f, "{}", it.seq)?;

            if idx != self.0.len() - 1 || !matches!(it.sep, Separator::Seq) {
                write!(f, "{}", it.sep)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundBlockItem {
    seq: PipelineSeq,
    sep: Separator,
}

impl CompoundBlockItem {
    pub fn new(seq: PipelineSeq, sep: Separator) -> Self {
        Self { seq, sep }
    }
}

impl std::fmt::Display for CompoundBlockItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.seq)?;
        write!(f, "{}", self.sep)?;
        Ok(())
    }
}

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn compound_block(&mut self) -> Option<CompoundBlock> {
        self.transaction(|parser| {
            parser.linebreak()?;

            let fst = parser.pipeline_seq()?;
            let rest = repeat!(|| {
                let sep = parser
                    .block_separator()
                    .map(|sep| sep.unwrap_or(Separator::Seq))?;
                let seq = parser.pipeline_seq()?;
                Some((sep, seq))
            });
            let last_sep = parser
                .block_separator()
                .unwrap_or(None)
                .unwrap_or(Separator::Seq);

            let (mut seps, mut seqs): (Vec<_>, Vec<_>) = rest.into_iter().unzip();

            seqs.insert(0, fst);
            seps.push(last_sep);

            let items = seqs
                .into_iter()
                .zip(seps)
                .map(|(seq, sep)| CompoundBlockItem { seq, sep })
                .collect();

            Some(CompoundBlock(items))
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Separator {
    /// The prev command runs asynchronously (`&`)
    Async,
    /// The prev command runs synchronously (`;`)
    Seq,
}

impl std::fmt::Display for Separator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Separator::Async => write!(f, "&"),
            Separator::Seq => write!(f, ";"),
        }
    }
}

impl Parser<'_> {
    /// a block can be delimited by either a separator followed by optional newlines,
    /// or by at least one newline
    #[tracing::instrument(skip(self), ret)]
    pub fn block_separator(&mut self) -> Option<Option<Separator>> {
        self.transaction(|parser| {
            one_of!(
                parser,
                || {
                    let sep = parser.cmd_separator()?;
                    parser.linebreak();
                    // the parser executed successfully and there exists a separator
                    Some(Some(sep))
                },
                || {
                    parser.required_linebreak()?;
                    // the parser executed successfully but there isn't a separator
                    Some(None)
                }
            )
        })
    }

    #[tracing::instrument(skip(self), ret)]
    pub fn cmd_separator(&mut self) -> Option<Separator> {
        self.transaction(|parser| {
            let res = one_of!(
                parser,
                || parser.consume(Symbol::Amp).and(Some(Separator::Async)),
                || parser.consume(Symbol::Semicolon).and(Some(Separator::Seq))
            )?;
            tracing::debug!("cmd separator {:?}", res);
            Some(res)
        })
    }
}
