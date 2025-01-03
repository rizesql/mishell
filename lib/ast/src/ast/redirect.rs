use crate::{ast, parser_v2::Parser, tokens::Symbol};

/// I/O redirection
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Redirect {
    /// Redirection to a file
    File(Option<ast::Fd>, Kind, Destination),
    /// Redirection both stdin and stderr (can append).
    OutAndErr(ast::Word, bool),
}

impl std::fmt::Display for Redirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Redirect::File(fd, kind, dest) => {
                if let Some(fd) = fd {
                    write!(f, "{fd}")?;
                }

                write!(f, "{kind} {dest}")
            }
            Redirect::OutAndErr(target, append) => {
                let sym = if *append { "&>>" } else { "&>" };
                write!(f, "{sym} {target}")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Read (`<`).
    Read,
    /// Write (`>`).
    Write,
    /// Append (`>>`).
    Append,
    /// Read and write (`<>`).
    ReadAndWrite,
    /// Clobber (`>|`).
    Clobber,
    /// Duplicate input (`<&`).
    DuplicateIn,
    /// Duplicate output (`>&`).
    DuplicateOut,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Read => write!(f, "<"),
            Kind::Write => write!(f, ">"),
            Kind::Append => write!(f, ">>"),
            Kind::ReadAndWrite => write!(f, "<>"),
            Kind::Clobber => write!(f, ">|"),
            Kind::DuplicateIn => write!(f, "<&"),
            Kind::DuplicateOut => write!(f, ">&"),
        }
    }
}

impl TryFrom<Symbol> for Kind {
    type Error = ();

    fn try_from(value: Symbol) -> Result<Self, Self::Error> {
        match value {
            Symbol::Lt => Ok(Kind::Read),
            Symbol::LtAnd => Ok(Kind::DuplicateIn),
            Symbol::Gt => Ok(Kind::Write),
            Symbol::GtAnd => Ok(Kind::DuplicateOut),
            Symbol::DoubleGt => Ok(Kind::Append),
            Symbol::LtGt => Ok(Kind::ReadAndWrite),
            Symbol::Clobber => Ok(Kind::Clobber),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Destination {
    Filename(ast::Word),
    Fd(ast::Fd),
}

impl std::fmt::Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Destination::Filename(word) => write!(f, "{word}"),
            Destination::Fd(fd) => write!(f, "{fd}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redirects(Vec<Redirect>);

impl From<Vec<Redirect>> for Redirects {
    fn from(value: Vec<Redirect>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Redirects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.0 {
            write!(f, "{r}")?;
        }

        Ok(())
    }
}

impl Parser<'_> {
    pub fn redirect(&mut self) -> Option<Redirect> {
        let fd = self.io_number();
        let kind = self.redirect_kind()?;
        let dest = self.redirect_dest()?;

        match (kind, &dest) {
            // destination must be a file name
            (
                Kind::Read | Kind::Write | Kind::Append | Kind::ReadAndWrite | Kind::Clobber,
                Destination::Filename(_),
            ) => Some(Redirect::File(fd, kind, dest)),

            // destination can be a file or a file descriptor
            (Kind::DuplicateIn | Kind::DuplicateOut, _) => Some(Redirect::File(fd, kind, dest)),
            // any other combination is not correct
            _ => None,
        }
    }

    pub fn redirects(&mut self) -> Option<Redirects> {
        let mut acc = Vec::new();
        while let Some(r) = self.redirect() {
            acc.push(r);
        }

        if !acc.is_empty() {
            Some(Redirects::from(acc))
        } else {
            None
        }
    }

    fn redirect_kind(&mut self) -> Option<Kind> {
        self.choice(|tok| tok.is_symbol()?.try_into().ok())
    }

    fn redirect_dest(&mut self) -> Option<Destination> {
        self.fd()
            .map(Destination::Fd)
            .or(self.filename().map(Destination::Filename))
    }

    fn filename(&mut self) -> Option<ast::Word> {
        self.choice(|tok| tok.is_word().map(ast::Word::from))
    }
}
