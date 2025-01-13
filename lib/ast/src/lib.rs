pub mod ast;
pub mod executables_cache;
mod lexer;
pub mod parser;
pub mod parser_v2;
pub mod tokenizer;
pub mod tokenizer_v2;
mod tokens;

pub(crate) const INDENT: &str = "    ";
