pub mod ast;
mod lexer;
mod parser;
pub mod parser_v2;
mod plm;
pub mod tokenizer;
pub mod tokenizer_v2;
mod tokens;

pub(crate) const INDENT: &str = "    ";
