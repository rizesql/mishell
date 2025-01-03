mod epxr;
mod for_clause;
mod operators;

pub use epxr::{Expr, UnexpandedExpr};
pub use for_clause::For;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arithmetic(UnexpandedExpr);

impl std::fmt::Display for Arithmetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(({}))", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    Variable(String),
    ArrayItem(String, Box<Expr>),
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Variable(ref v) => write!(f, "{v}"),
            Target::ArrayItem(ref v, idx) => write!(f, "{v}[{idx}]"),
        }
    }
}
