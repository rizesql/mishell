use crate::{ast, tokens::Token};

pub trait Parse<T> {
    fn parse(&mut self) -> Option<T>;
}

pub struct Parser<'a> {
    pos: usize,
    src: &'a [Token],
}

impl<'p> Parser<'p> {
    pub fn new(src: &'p [Token]) -> Self {
        Self { src, pos: 0 }
    }

    pub fn peek(&self) -> Option<&'p Token> {
        self.src.get(self.pos)
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn consume(&mut self, expected: impl Into<Token>) -> Option<()> {
        if self.peek() != Some(&expected.into()) {
            return None;
        }

        self.advance();
        Some(())
    }

    pub fn remaining(&self) -> &'p [Token] {
        &self.src[self.pos..]
    }

    pub fn choice<T>(&mut self, mut matcher: impl FnMut(&'p Token) -> Option<T>) -> Option<T> {
        let token = self.peek()?;
        let res = matcher(token)?;

        self.advance();
        Some(res)
    }

    pub fn repeat<T>(&mut self, mut matcher: impl FnMut(&'p Token) -> Option<T>) -> Option<Vec<T>> {
        let mut acc = Vec::new();

        while let Some(r) = self.choice(&mut matcher) {
            acc.push(r);
        }

        Some(acc)
    }

    pub fn at_least_once<T>(
        &mut self,
        mut matcher: impl FnMut(&'p Token) -> Option<T>,
    ) -> Option<Vec<T>> {
        self.repeat(&mut matcher).filter(|r| r.len() > 0)
    }

    pub fn word(&mut self) -> Option<ast::Word> {
        self.choice(|tok| tok.is_word()).map(ast::Word::from)
    }

    pub fn io_number(&mut self) -> Option<u32> {
        let word = self.word()?;
        if !word.raw().chars().all(|c| char::is_ascii_digit(&c)) {
            return None;
        }

        let sym = self.peek()?.is_symbol()?;
        let op = sym.as_str();

        if op.starts_with('<') || op.starts_with('>') {
            word.raw().parse().ok()
        } else {
            None
        }
    }
}
