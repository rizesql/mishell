use crate::{
    one_of,
    parser_v2::Parser,
    repeat,
    tokens::{Paren, Symbol, Token},
};

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

impl Parser<'_> {
    #[tracing::instrument(skip(self), ret)]
    pub fn arithmetic_expr(&mut self) -> Option<UnexpandedExpr> {
        self.transaction(|parser| {
            let value = repeat!(|| parser.expr_part()).concat();
            if !value.is_empty() {
                Some(UnexpandedExpr { value })
            } else {
                None
            }
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn expr_part(&mut self) -> Option<String> {
        self.transaction(|parser| {
            one_of!(
                parser,
                || {
                    parser.consume(Paren::open())?;
                    // let

                    let expr = repeat!(|| {
                        if parser.peek()? == &Paren::close().into() {
                            None
                        } else {
                            parser.expr_part()
                        }
                    })
                    .concat();
                    parser.consume(Paren::close())?;
                    Some(format!("({expr})"))
                },
                || {
                    if parser.arithmetic_end() {
                        return None;
                    }

                    let t = parser.consume_any()?;
                    tracing::info!("\n\n\nCONSUMED {t}");
                    Some(t.as_str().to_owned())
                }
            )
        })
    }

    #[tracing::instrument(skip(self), ret)]
    fn arithmetic_end(&mut self) -> bool {
        self.transaction(|parser| {
            one_of!(
                parser,
                || {
                    parser.consume(Paren::close())?;
                    parser.consume(Paren::close())?;
                    Some(())
                },
                || { parser.consume(Symbol::Semicolon) }
            )
        })
        .is_some()
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
