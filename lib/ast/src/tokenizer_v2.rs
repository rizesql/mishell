use crate::tokens::Token;

use std::{
    iter::{Fuse, Peekable},
    str::FromStr,
};

use crate::tokens::{Paren, Positional, Quote, Symbol};

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenOrLiteral {
    Token(Token),
    Escaped(Option<Token>),
    Literal(char),
}

pub struct Lexer<I: Iterator<Item = char>> {
    inner: Peekable<Fuse<I>>,
    peeked: Option<TokenOrLiteral>,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    pub fn new(iter: I) -> Lexer<I> {
        Self {
            inner: iter.fuse().peekable(),
            peeked: None,
        }
    }

    fn next_is(&mut self, c: char) -> bool {
        let cond = self.inner.peek() == Some(&c);

        if cond {
            self.inner.next();
        }

        cond
    }

    fn __next(&mut self) -> Option<TokenOrLiteral> {
        if self.peeked.is_some() {
            return self.peeked.take();
        }

        let curr = self.inner.next()?;

        let token = match curr {
            '\n' => Token::NewLine,
            '!' => Token::Symbol(Symbol::Bang),
            '#' => Token::Symbol(Symbol::Pound),
            '*' => Token::Symbol(Symbol::Star),
            '?' => Token::Symbol(Symbol::Question),
            '%' => Token::Symbol(Symbol::Percent),
            '-' => Token::Symbol(Symbol::Dash),
            '=' => Token::Symbol(Symbol::Equals),
            '+' => Token::Symbol(Symbol::Plus),
            ':' => Token::Symbol(Symbol::Colon),
            '@' => Token::Symbol(Symbol::At),
            '^' => Token::Symbol(Symbol::Caret),
            '/' => Token::Symbol(Symbol::Slash),
            ',' => Token::Symbol(Symbol::Comma),

            // Make sure that we treat the next token as a single character,
            // preventing multi-char tokens from being recognized. This is
            // important because something like `\&&` would mean that the
            // first & is a literal while the second retains its properties.
            // We will let the parser deal with what actually becomes a literal.
            '\\' => {
                return Some(TokenOrLiteral::Escaped(
                    self.inner
                        .next()
                        .and_then(|c| Lexer::new(std::iter::once(c)).next()),
                ))
            }

            '\'' => Token::Quote(Quote::Single),
            '"' => Token::Quote(Quote::Double),
            '`' => Token::Quote(Quote::Backtick),

            ';' => Token::Symbol(if self.next_is(';') {
                Symbol::DoubleSemi
            } else {
                Symbol::Semicolon
            }),

            '&' => Token::Symbol(if self.next_is('&') {
                Symbol::And
            } else {
                Symbol::Amp
            }),

            '|' => Token::Symbol(if self.next_is('|') {
                Symbol::Or
            } else {
                Symbol::Pipe
            }),

            '(' | ')' | '[' | ']' | '{' | '}' => {
                let mut buffer = [0; 4];
                let s = curr.encode_utf8(&mut buffer);
                Token::Paren(Paren::from_str(s).unwrap())
            }

            '$' => match self
                .inner
                .peek()
                .and_then(|c| Positional::try_from(*c).ok())
            {
                Some(p) => {
                    self.inner.next();
                    Token::PositionalParam(p)
                }
                None => Token::Symbol(Symbol::Dollar),
            },

            '<' => Token::Symbol(if self.next_is('<') {
                if self.next_is('-') {
                    Symbol::DoubleLtDash
                } else {
                    Symbol::DoubleLt
                }
            } else if self.next_is('&') {
                Symbol::LtAnd
            } else if self.next_is('>') {
                Symbol::LtGt
            } else {
                Symbol::Lt
            }),

            '>' => Token::Symbol(if self.next_is('>') {
                Symbol::DoubleGt
            } else if self.next_is('&') {
                Symbol::GtAnd
            } else if self.next_is('|') {
                Symbol::Clobber
            } else {
                Symbol::Gt
            }),

            c if c.is_whitespace() => {
                let mut buf = String::new();
                buf.push(c);

                while let Some(&c) = self.inner.peek() {
                    if c.is_whitespace() && c != '\n' {
                        self.inner.next();
                        buf.push(c);
                    } else {
                        break;
                    }
                }

                Token::Whitespace(buf)
            }

            c => return Some(TokenOrLiteral::Literal(c)),
        };

        Some(TokenOrLiteral::Token(token))
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        fn word_first_char(c: char) -> bool {
            c == '_' || c.is_alphabetic()
        }

        fn word_char(c: char) -> bool {
            c.is_ascii_digit() || word_first_char(c)
        }

        match self.__next() {
            None => None,
            Some(TokenOrLiteral::Token(t)) => Some(t),
            Some(TokenOrLiteral::Escaped(t)) => {
                debug_assert_eq!(self.peeked, None);
                self.peeked = t.map(TokenOrLiteral::Token);
                Some(Token::Symbol(Symbol::Backslash))
            }
            Some(TokenOrLiteral::Literal(t)) => {
                let is_word = word_first_char(t);
                let mut word = String::new();
                word.push(t);

                loop {
                    match self.__next() {
                        Some(tok @ TokenOrLiteral::Token(_))
                        | Some(tok @ TokenOrLiteral::Escaped(_)) => {
                            debug_assert_eq!(self.peeked, None);
                            self.peeked = Some(tok);
                            break;
                        }
                        Some(TokenOrLiteral::Literal(c)) if is_word && !word_char(c) => {
                            debug_assert_eq!(self.peeked, None);
                            self.peeked = Some(TokenOrLiteral::Literal(c));
                            return Some(Token::Word(word));
                        }
                        Some(TokenOrLiteral::Literal(c)) => word.push(c),
                        None => break,
                    }
                }

                Some(if is_word {
                    Token::Word(word)
                } else {
                    // Token::Literal(word)
                    Token::Word(word)
                })
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, hi) = self.inner.size_hint();
        let low = usize::from(self.peeked.is_some());
        (low, hi)
    }
}
