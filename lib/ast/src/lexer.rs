// use utf8_chars::BufReadCharsExt;

// use crate::tokens::{Quote, Token};

// #[derive(Debug, Clone, Copy)]
// pub(crate) enum TokenEnd {
//     /// Reached end of input
//     EOI,

//     /// Unescaped newline character
//     UnescapedNewline,

//     /// Terminating char
//     TerminatingChar,

//     /// Operator start
//     OpStart,
//     /// Operator terminated
//     OpEnd,

//     /// Other condition was reached
//     Other,
// }

// #[derive(Debug, Clone, Default, Copy)]
// pub(crate) struct Position {
//     pub idx: u32,
//     pub row: u32,
//     pub col: u32,
// }

// impl std::fmt::Display for Position {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "row: {} col: {}", self.row, self.col)
//     }
// }

// #[derive(Debug, Clone, Default, Copy)]
// struct TokenLocation {
//     pub start: Position,
//     pub end: Position,
// }

// #[derive(Debug, Clone)]
// pub(crate) struct LexerResult {
//     end_reason: TokenEnd,
//     token: Option<Token>,
// }

// #[derive(Debug, thiserror::Error)]
// pub(crate) enum Error {
//     #[error("Unterminated escape sequence")]
//     UnterminatedEscape,

//     #[error("Unterminated {0}")]
//     UnterminatedQuote(Quote),

//     #[error("Unterminated command substitution")]
//     UnterminatedCommandSubstitution,

//     #[error("Failed to decode input")]
//     Decoding,

//     #[error("Failed to read input")]
//     Io(#[from] std::io::Error),
// }

// #[derive(Debug, Clone)]
// struct LexerState {
//     cursor: Position,
//     queue: Vec<LexerResult>,
//     arithmetic_mode: bool,
// }

// pub(crate) struct Lexer<'l, R: ?Sized + std::io::BufRead> {
//     reader: std::iter::Peekable<utf8_chars::Chars<'l, R>>,
//     state: LexerState,
// }

// #[derive(Debug, Clone)]
// struct TokenState {
//     pub start: Position,
//     pub acc: String,
//     pub is_symbol: bool,
//     pub escaping: bool,
//     pub quotes: Option<Quote>,
// }

// impl TokenState {
//     pub fn new(start: Position) -> Self {
//         Self {
//             start,
//             acc: String::new(),
//             is_symbol: false,
//             escaping: false,
//             quotes: None,
//         }
//     }

//     pub fn pop(&mut self, end: &Position) -> Token {
//         let tok = if self.is_symbol {
//             todo!()
//             // Token::Symbol(self.acc.into())
//         } else {
//             Token::Word(std::mem::take(&mut self.acc))
//         };

//         self.start = *end;
//         self.escaping = false;
//         self.quotes = None;

//         tok
//     }

//     pub fn has_acc(&self) -> bool {
//         !self.acc.is_empty()
//     }

//     pub fn acc_char(&mut self, c: char) {
//         self.acc.push(c)
//     }

//     pub fn acc_str(&mut self, s: &str) {
//         self.acc.push_str(s);
//     }

//     pub fn unquoted(&self) -> bool {
//         !self.escaping && self.quotes.is_none()
//     }

//     pub fn acc(&self) -> &str {
//         &self.acc
//     }

//     pub fn delimit_acc(&mut self, end_reason: TokenEnd, state: &mut LexerState) -> LexerResult {
//         LexerResult {
//             end_reason,
//             token: match self.has_acc() {
//                 true => Some(self.pop(&state.cursor)),
//                 false => None,
//             },
//         }
//     }
// }

// impl<'r, R: ?Sized + std::io::BufRead> Lexer<'r, R> {
//     pub fn new(reader: &'r mut R) -> Self {
//         Self {
//             reader: reader.chars().peekable(),
//             state: LexerState {
//                 cursor: Position {
//                     idx: 0,
//                     row: 1,
//                     col: 1,
//                 },
//                 queue: vec![],
//                 arithmetic_mode: false,
//             },
//         }
//     }

//     fn next(&mut self) -> Result<Option<char>, Error> {
//         let c = self.reader.next().transpose().map_err(Error::Io)?;

//         if let Some(c) = c {
//             self.state.cursor.idx += 1;
//             if c == '\n' {
//                 self.state.cursor.row += 1;
//                 self.state.cursor.col = 1;
//             } else {
//                 self.state.cursor.col += 1;
//             }
//         }

//         Ok(c)
//     }

//     fn consume(&mut self) -> Result<(), Error> {
//         let _ = self.next()?;
//         Ok(())
//     }

//     fn peek(&mut self) -> Result<Option<char>, Error> {
//         match self.reader.peek() {
//             None => Ok(None),
//             Some(r) => match r {
//                 Ok(c) => Ok(Some(*c)),
//                 Err(_) => Err(Error::Decoding),
//             },
//         }
//     }

//     fn next_until(&mut self, termination: Option<char>) -> Result<LexerResult, Error> {
//         let mut state = TokenState::new(self.state.cursor);
//         let mut res = None;

//         while res.is_none() {
//             if !self.state.queue.is_empty() {
//                 return Ok(self.state.queue.remove(0));
//             }

//             let next = self.peek()?;
//             let ch = next.unwrap_or('\0');

//             if next.is_none() {
//                 if state.escaping {
//                     return Err(Error::UnterminatedEscape);
//                 }

//                 if let Some(q) = state.quotes {
//                     return Err(Error::UnterminatedQuote(q));
//                 }

//                 res = Some(state.delimit_acc(TokenEnd::EOI, &mut self.state));
//             } else if state.unquoted() && termination == Some(ch) {
//                 res = Some(state.delimit_acc(TokenEnd::TerminatingChar, &mut self.state));
//             } else if state.is_symbol {
//                 let mut curr = state.acc().to_owned();
//                 curr.push(ch);

//                 if state.unquoted() && self.is_operator(curr.as_ref()) {
//                     self.consume()?;
//                     state.acc_char(ch);
//                 } else {
//                     assert!(state.has_acc());

//                     let end_reason = if state.acc() == "\n" {
//                         TokenEnd::UnescapedNewline
//                     } else {
//                         TokenEnd::OpEnd
//                     };

//                     res = Some(state.delimit_acc(end_reason, &mut self.state));
//                 }
//             } else if
//         }

//         todo!()
//     }

//     fn is_operator(&self, s: &str) -> bool {
//         matches!(
//             s,
//             "&" | "&&"
//                 | "("
//                 | ")"
//                 | ";"
//                 | ";;"
//                 | "\n"
//                 | "|"
//                 | "||"
//                 | "<"
//                 | ">"
//                 | ">|"
//                 | "<<"
//                 | ">>"
//                 | "<&"
//                 | ">&"
//                 | "<>"
//         )
//     }
// }
