use std::str::FromStr;

/// Inner representation of a positional param.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Positional {
    /// $0
    Zero,
    /// $1
    One,
    /// $2
    Two,
    /// $3
    Three,
    /// $4
    Four,
    /// $5
    Five,
    /// $6
    Six,
    /// $7
    Seven,
    /// $8
    Eight,
    /// $9
    Nine,
}

impl Positional {
    fn as_str(&self) -> &str {
        match self {
            Positional::Zero => "$0",
            Positional::One => "$1",
            Positional::Two => "$2",
            Positional::Three => "$3",
            Positional::Four => "$4",
            Positional::Five => "$5",
            Positional::Six => "$6",
            Positional::Seven => "$7",
            Positional::Eight => "$8",
            Positional::Nine => "$9",
        }
    }
}

impl TryFrom<char> for Positional {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            _ => Err("value is not a positional param".into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParenKind {
    /// `(` or `)`
    Normal,
    /// `{` or `}`
    Curly,
    /// `[` or `]`
    Square,
}

impl FromStr for ParenKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" | ")" => Ok(Self::Normal),
            "[" | "]" => Ok(Self::Square),
            "{" | "}" => Ok(Self::Curly),
            _ => Err("Char is not a paren".into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParenPos {
    Open,
    Close,
}

impl FromStr for ParenPos {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" | "[" | "{" => Ok(Self::Open),
            ")" | "]" | "}" => Ok(Self::Close),
            _ => Err("Char is not a paren".into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paren {
    kind: ParenKind,
    pos: ParenPos,
}

impl Paren {
    fn as_str(&self) -> &str {
        match (self.kind, self.pos) {
            (ParenKind::Normal, ParenPos::Open) => "(",
            (ParenKind::Normal, ParenPos::Close) => ")",
            (ParenKind::Curly, ParenPos::Open) => "{",
            (ParenKind::Curly, ParenPos::Close) => "}",
            (ParenKind::Square, ParenPos::Open) => "[",
            (ParenKind::Square, ParenPos::Close) => "]",
        }
    }
}

impl FromStr for Paren {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            kind: ParenKind::from_str(s)?,
            pos: ParenPos::from_str(s)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quote {
    /// '
    Single,
    /// "
    Double,
    /// `
    Backtick,
}

impl Quote {
    fn as_str(&self) -> &str {
        match self {
            Quote::Single => "'",
            Quote::Double => "\"",
            Quote::Backtick => "`",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    /// \!
    Bang,
    /// \~
    Tilde,
    /// \#
    Pound,
    /// \*
    Star,
    /// \?
    Question,
    /// \\
    Backslash,
    /// \%
    Percent,
    /// \-
    Dash,
    /// \=
    Equals,
    /// \+
    Plus,
    /// \:
    Colon,
    /// \@
    At,
    /// \^
    Caret,
    /// \/
    Slash,
    /// \,
    Comma,
    /// \;
    Semicolon,
    /// \&
    Amp,
    /// \|
    Pipe,
    /// \&&
    And,
    /// \||
    Or,
    /// \;;
    DoubleSemi,
    /// \$
    Dollar,
    /// \<
    Lt,
    /// \>
    Gt,
    /// <<
    DoubleLt,
    /// \>>
    DoubleGt,
    /// <&
    LtAnd,
    /// \>&
    GtAnd,
    /// <<-
    DoubleLtDash,
    /// \>|
    Clobber,
    /// <>
    LtGt,
}

impl Symbol {
    pub fn as_str(&self) -> &str {
        match self {
            Symbol::Bang => "!",
            Symbol::Tilde => "~",
            Symbol::Pound => "#",
            Symbol::Star => "*",
            Symbol::Question => "?",
            Symbol::Backslash => "\\",
            Symbol::Percent => "%",
            Symbol::Dash => "-",
            Symbol::Equals => "=",
            Symbol::Plus => "+",
            Symbol::Colon => ":",
            Symbol::At => "@",
            Symbol::Caret => "^",
            Symbol::Slash => "/",
            Symbol::Comma => ",",
            Symbol::Semicolon => ";",
            Symbol::Amp => "&",
            Symbol::Pipe => "|",
            Symbol::And => "&&",
            Symbol::Or => "||",
            Symbol::DoubleSemi => ";;",
            Symbol::Dollar => "$",
            Symbol::Lt => "<",
            Symbol::Gt => ">",
            Symbol::DoubleLt => "<<",
            Symbol::DoubleGt => ">>",
            Symbol::GtAnd => ">&",
            Symbol::LtAnd => "<&",
            Symbol::DoubleLtDash => "<<-",
            Symbol::Clobber => ">|",
            Symbol::LtGt => "<>",
        }
    }
}

impl From<Symbol> for Token {
    fn from(value: Symbol) -> Self {
        Self::Symbol(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    NewLine,
    Paren(Paren),
    Symbol(Symbol),
    Quote(Quote),
    PositionalParam(Positional),
    Whitespace(String),
    Literal(String),
    Word(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Token {
    fn as_str(&self) -> &str {
        match self {
            Token::NewLine => "\n",
            Token::Paren(paren) => paren.as_str(),
            Token::Symbol(symbol) => symbol.as_str(),
            Token::Quote(quote) => quote.as_str(),
            Token::PositionalParam(positional) => positional.as_str(),
            Token::Whitespace(ref s) => s,
            Token::Literal(ref s) => s,
            Token::Word(ref s) => s,
        }
    }

    pub fn is_word(&self) -> Option<String> {
        match self {
            Token::Word(w) => Some(w.to_owned()),
            _ => None,
        }
    }

    pub fn is_symbol(&self) -> Option<Symbol> {
        match self {
            Token::Symbol(s) => Some(*s),
            _ => None,
        }
    }
}
