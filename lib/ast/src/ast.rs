mod command;
mod pipeline;
mod process_substitution;
mod redirect;

pub use command::{compound::CompoundCommand, Command};
pub use pipeline::{Pipeline, PipelineSeq};
pub use process_substitution::ProcSubstitution;
pub use redirect::{Redirect, Redirects};

use crate::parser_v2::Parser;

pub type Fd = u32;

impl Parser<'_> {
    fn fd(&mut self) -> Option<Fd> {
        self.choice(|tok| tok.is_word()?.parse().ok())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    raw: String,
}

impl Word {
    pub fn raw(&self) -> &String {
        &self.raw
    }
}

impl From<String> for Word {
    fn from(raw: String) -> Self {
        Self { raw }
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}
