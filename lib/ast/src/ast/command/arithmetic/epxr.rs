use super::{operators, Target};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnexpandedExpr {
    value: String,
}

impl std::fmt::Display for UnexpandedExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Literal(i64),
    Ref(Target),
    UnaryOp {
        op: operators::Unary,
        this: Box<Expr>,
    },
    BinaryOp {
        op: operators::Binary,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Cond {
        cond: Box<Expr>,
        then: Box<Expr>,
        otherwise: Box<Expr>,
    },
    Assign {
        lhs: Target,
        rhs: Box<Expr>,
    },
    OpAssign {
        op: operators::Binary,
        lhs: Target,
        rhs: Box<Expr>,
    },
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(v) => write!(f, "{v}"),
            Expr::Ref(target) => write!(f, "{target}"),
            Expr::UnaryOp { op, this } => write!(f, "{op}{this}"),
            Expr::BinaryOp { op, lhs, rhs } => write!(f, "{lhs} {op} {rhs}"),
            Expr::Cond {
                cond,
                then,
                otherwise,
            } => write!(f, "{cond} ? {then} : {otherwise}"),
            Expr::Assign { lhs, rhs } => write!(f, "{lhs} = {rhs}"),
            Expr::OpAssign { op, lhs, rhs } => write!(f, "{lhs} {op}= {rhs}"),
        }
    }
}
