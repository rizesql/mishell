use crate::{ast, delimited_repeat, one_of, parser_v2::Parser, repeat, tokens::Symbol};

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

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn pipeline(&mut self) -> Option<Pipeline> {
        self.transaction(|parser| {
            let bang = parser.consume(Symbol::Bang).is_some();
            let seq = parser.command_seq()?;
            let res = Pipeline { bang, seq };

            Some(res)
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn command_seq(&mut self) -> Option<Vec<ast::Command>> {
        self.transaction(|parser| {
            delimited_repeat!(|| parser.command(), || parser.pipeline_separator())
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn pipeline_separator(&mut self) -> Option<()> {
        self.transaction(|parser| {
            parser.consume(Symbol::Pipe)?;
            parser.linebreak()
        })
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

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn pipeline_seq(&mut self) -> Option<PipelineSeq> {
        self.transaction(|parser| {
            let head = parser.pipeline()?;
            let rest = repeat!(|| parser.seq_step());

            let res = PipelineSeq { head, rest };
            Some(res)
        })
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

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    fn seq_step(&mut self) -> Option<SeqStep> {
        self.transaction(|parser| one_of!(parser, || parser.or_step(), || parser.and_step()))
    }

    #[tracing::instrument(skip(self), ret)]
    fn or_step(&mut self) -> Option<SeqStep> {
        self.transaction(|parser| {
            parser.consume(Symbol::Or)?;
            let pipeline = parser.pipeline()?;
            let res = SeqStep::Or(pipeline);

            Some(res)
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn and_step(&mut self) -> Option<SeqStep> {
        self.transaction(|parser| {
            parser.consume(Symbol::And)?;
            let pipeline = parser.pipeline()?;
            let res = SeqStep::And(pipeline);
            Some(res)
        })
    }
}
