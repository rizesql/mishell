use crate::tokens::{Quote, Token};

pub trait Parse<T> {
    fn parse(&mut self) -> Option<T>;
}

#[macro_export]
macro_rules! repeat {
    ($fn:expr) => {{
        let mut acc = Vec::new();
        while let Some(it) = ($fn)() {
            acc.push(it);
        }
        acc
    }};
}

#[macro_export]
macro_rules! delimited_repeat {
    ($fn:expr, $delim:expr) => {{
        let mut acc = Vec::new();

        if let Some(it) = ($fn)() {
            acc.push(it);
        } else {
            return None;
        }

        while let Some(_) = ($delim)() {
            if let Some(it) = ($fn)() {
                acc.push(it);
            } else {
                break;
            }
        }

        Some(acc)
    }};
}

#[macro_export]
macro_rules! min_once {
    ($fn:expr) => {{
        let mut acc = Vec::new();

        while let Some(it) = ($fn)() {
            acc.push(it);
        }

        if !acc.is_empty() {
            Some(acc)
        } else {
            None
        }
    }};
}

#[macro_export]
macro_rules! one_of {
    ($parser:expr, $fn:expr) => {{
        ($fn)()
    }};

    ($parser:expr, $fn:expr, $($rest:expr),*) => {{
        let pos = $parser.pos();
        if let Some(res) = ($fn)() {
            return Some(res);
        }

        $parser.reset_pos(pos);
        one_of!($parser, $($rest),*)
    }};
}

#[derive(Debug)]
pub struct Parser<'a> {
    pos: usize,
    src: &'a [Token],
    quoting: Option<Quote>,
}

impl<'p> Parser<'p> {
    pub fn new(src: &'p [Token]) -> Self {
        Self {
            src,
            pos: 0,
            quoting: None,
        }
    }

    pub fn peek(&self) -> Option<&'p Token> {
        self.src.get(self.pos)
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn reset_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn set_quoting(&mut self, quote: Option<Quote>) {
        self.quoting = quote;
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn consume_any(&mut self) -> Option<&Token> {
        self.transaction(|parser| {
            let t = parser.peek()?;
            parser.advance();
            Some(t)
        })
    }

    pub fn consume(&mut self, expected: impl Into<Token>) -> Option<()> {
        self.transaction(|parser| {
            if parser.peek() != Some(&expected.into()) {
                return None;
            }

            parser.advance();
            Some(())
        })
    }

    pub fn remaining(&self) -> &'p [Token] {
        &self.src[self.pos..]
    }

    pub fn choice<T>(&mut self, mut matcher: impl FnMut(&'p Token) -> Option<T>) -> Option<T> {
        self.transaction(|parser| {
            let tok = parser.peek()?;
            matcher(tok).map(|r| {
                parser.advance();
                r
            })
        })
    }

    pub fn transaction<T>(&mut self, cb: impl FnOnce(&mut Self) -> Option<T>) -> Option<T> {
        let snapshot = self.pos;

        while self.quoting.is_none() && matches!(self.peek(), Some(Token::Whitespace(_))) {
            self.set_quoting(match self.peek() {
                Some(Token::Quote(q)) => Some(*q),
                _ => None,
            });
            self.advance();
        }

        cb(self).or_else(|| {
            self.reset_pos(snapshot);
            None
        })
    }
}
