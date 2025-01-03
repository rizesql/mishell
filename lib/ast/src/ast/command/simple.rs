use crate::ast;

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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Affix {
    Redirect(ast::Redirect),
    Word(ast::Word),
    Assignment(Assignment, ast::Word),
    ProcSubstitution(ast::ProcSubstitution),
}

impl std::fmt::Display for Affix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Affix::Redirect(target) => write!(f, "{target}"),
            Affix::Word(target) => write!(f, "{target}"),
            Affix::Assignment(_, target) => write!(f, "{target}"),
            Affix::ProcSubstitution(sub) => {
                write!(f, "{}({})", sub.kind(), sub.command())
            }
        }
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
struct Assignment {
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
