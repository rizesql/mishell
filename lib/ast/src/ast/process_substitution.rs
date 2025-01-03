use super::command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcSubstitution {
    kind: Kind,
    command: command::compound::Subshell,
}

impl ProcSubstitution {
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn command(&self) -> &command::compound::Subshell {
        &self.command
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// the process is read from.
    Read,
    /// the process is written to.
    Write,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Read => write!(f, "<"),
            Kind::Write => write!(f, ">"),
        }
    }
}
