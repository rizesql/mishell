#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unary {
    /// Unary plus, ex: `+x`
    Add,
    /// Unary minus, ex: `-x`
    Sub,
    /// Logical not, ex: `!x`
    Not,
    /// Bitwise not, ex: `~x`
    BitNot,
}

impl std::fmt::Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unary::Add => write!(f, "+"),
            Unary::Sub => write!(f, "-"),
            Unary::Not => write!(f, "!"),
            Unary::BitNot => write!(f, "~"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Binary {
    /// Addition, ex: `x + y`
    Add,
    /// Subtraction, ex: `x - y`
    Sub,
    /// Multiplication, ex: `x * y`
    Mul,
    /// Division, ex: `x / y`
    Div,
    /// Modulo, ex: `x % y`
    Mod,
    /// Exponentiation, ex: `x ** y`
    Pow,
    /// Comma, ex: `x, y`
    Comma,
    /// Lower than, ex: `x < y`
    Lt,
    /// Lower than or equal to, ex: `x <= y`
    Lte,
    /// Lower than, ex: `x < y`
    Gt,
    /// Lower than or equal to, ex: `x <= y`
    Gte,
    /// Equal to, ex: `x == y`
    Eq,
    /// Not equal to, ex: `x != y`
    Ne,
    /// Logical and, ex: `x && y`
    And,
    /// Logical or, ex: `x || y`
    Or,
    /// Bitwise and, ex: `x & y`
    BitAnd,
    /// Bitwise or, ex: `x | y`
    BitOr,
    /// Bitwise xor, ex: `x ^ y`
    BitXor,
    /// Bitwise left shift, ex: `x << y`
    ShiftL,
    /// Bitwise right shift, ex: `x >> y`
    ShiftR,
}

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Binary::Add => write!(f, "+"),
            Binary::Sub => write!(f, "-"),
            Binary::Mul => write!(f, "*"),
            Binary::Div => write!(f, "/"),
            Binary::Mod => write!(f, "%"),
            Binary::Pow => write!(f, "**"),
            Binary::Comma => write!(f, ","),
            Binary::Lt => write!(f, "<"),
            Binary::Lte => write!(f, "<="),
            Binary::Gt => write!(f, ">"),
            Binary::Gte => write!(f, ">="),
            Binary::Eq => write!(f, "=="),
            Binary::Ne => write!(f, "!="),
            Binary::And => write!(f, "&&"),
            Binary::Or => write!(f, "||"),
            Binary::BitAnd => write!(f, "&"),
            Binary::BitOr => write!(f, "|"),
            Binary::BitXor => write!(f, "^"),
            Binary::ShiftL => write!(f, "<<"),
            Binary::ShiftR => write!(f, ">>"),
        }
    }
}
