mod command;
mod pipeline;
mod program;
mod redirect;
mod word;

pub use command::{compound::CompoundCommand, Command};
pub use pipeline::{Pipeline, PipelineSeq};
pub use redirect::{Redirect, Redirects};
pub use word::Word;

use crate::{parser_v2::Parser, tokens::Token};

pub type Fd = u32;

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn linebreak(&mut self) -> Option<()> {
        self.transaction(|parser| {
            while parser.consume(Token::NewLine).is_some() {}
            Some(())
        })
    }

    #[tracing::instrument(skip(self), ret)]
    pub fn required_linebreak(&mut self) -> Option<()> {
        self.transaction(|parser| {
            parser.consume(Token::NewLine)?;
            parser.linebreak()
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn fd(&mut self) -> Option<Fd> {
        self.transaction(|parser| {
            let res = parser.choice(|tok| tok.as_word()?.parse().ok())?;

            Some(res)
        })
    }

    #[tracing::instrument(skip(self), ret)]
    pub fn io_number(&mut self) -> Option<u32> {
        self.transaction(|parser| {
            let word = parser.word()?;
            if !word.raw().chars().all(|c| char::is_ascii_digit(&c)) {
                return None;
            }

            let sym = parser.peek()?.as_symbol()?;
            let op = sym.as_str();

            if op.starts_with('<') || op.starts_with('>') {
                let res = word.raw().parse().ok()?;
                tracing::debug!("io_number {:?}", res);
                Some(res)
            } else {
                None
            }
        })
    }
}
