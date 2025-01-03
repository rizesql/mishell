use crate::ast;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pipeline {
    bang: bool,
    seq: Vec<ast::Command>,
}

impl std::fmt::Display for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.bang {
            write!(f, "!")?;
        }

        let mut seq = self.seq.iter();
        if let Some(fst) = seq.next() {
            write!(f, "{fst}")?;

            for cmd in seq {
                write!(f, " | {cmd}")?;
            }
        }

        Ok(())
    }
}

/// Represents a sequence of command pipelines connected through boolean operations
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineSeq {
    head: Pipeline,
    rest: Vec<SeqStep>,
}

impl std::fmt::Display for PipelineSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.head)?;

        for it in &self.rest {
            write!(f, "{it}")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SeqStep {
    And(Pipeline),
    Or(Pipeline),
}

impl std::fmt::Display for SeqStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeqStep::And(p) => write!(f, " && {p}"),
            SeqStep::Or(p) => write!(f, " || {p}"),
        }
    }
}
