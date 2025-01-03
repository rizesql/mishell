use crate::ast::command::compound::DoScope;

use super::UnexpandedExpr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct For {
    init: Option<UnexpandedExpr>,
    cond: Option<UnexpandedExpr>,
    post: Option<UnexpandedExpr>,
    block: DoScope,
}

impl std::fmt::Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "for ((")?;

        if let Some(init) = &self.init {
            write!(f, "{init}")?;
        }

        write!(f, "; ")?;

        if let Some(cond) = &self.cond {
            write!(f, "{cond}")?;
        }

        write!(f, "; ")?;

        if let Some(post) = &self.post {
            write!(f, "{post}")?;
        }

        write!(f, "))")?;

        write!(f, "{}", self.block)
    }
}
