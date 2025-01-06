use crate::{delimited_repeat, one_of, parser_v2::Parser, repeat};

use super::{
    command::{self, compound::Separator},
    PipelineSeq,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    cmds: Vec<CompleteCommand>,
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cmd in &self.cmds {
            write!(f, "{cmd}")?;
        }
        Ok(())
    }
}

pub type CompleteCommand = command::compound::CompoundBlock;

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn program(&mut self) -> Option<Program> {
        self.transaction(|parser| {
            one_of!(
                parser,
                || {
                    parser.linebreak()?;
                    let cmds = parser.complete_commands()?;
                    parser.linebreak()?;

                    Some(Program { cmds })
                },
                || {
                    parser.linebreak()?;
                    Some(Program { cmds: vec![] })
                }
            )
        })
    }

    #[tracing::instrument(skip(self), ret)]
    pub fn complete_commands(&mut self) -> Option<Vec<CompleteCommand>> {
        self.transaction(|parser| {
            let res =
                delimited_repeat!(|| parser.complete_command(), || parser.required_linebreak())?;
            Some(res)
        })
    }

    #[tracing::instrument(skip(self), ret)]
    pub fn complete_command(&mut self) -> Option<CompleteCommand> {
        self.transaction(|parser| {
            let fst = parser.pipeline_seq()?;

            let rest = repeat!(|| parser.command_part());

            let last_sep = parser
                .cmd_separator()
                .unwrap_or(command::compound::Separator::Seq);

            let (mut seps, mut seqs): (Vec<_>, Vec<_>) = rest.into_iter().unzip();

            seqs.insert(0, fst);
            seps.push(last_sep);

            let items = seqs
                .into_iter()
                .zip(seps)
                .map(|(seq, sep)| command::compound::CompoundBlockItem::new(seq, sep))
                .collect();

            let res = command::compound::CompoundBlock::new(items);
            Some(res)
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn command_part(&mut self) -> Option<(Separator, PipelineSeq)> {
        self.transaction(|parser| {
            let sep = parser.cmd_separator()?;
            let seq = parser.pipeline_seq()?;
            Some((sep, seq))
        })
    }
}
