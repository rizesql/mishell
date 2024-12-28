mod tokenizer;
mod parser;

use std::io::{self, Write};
use tokenizer::tokenizer;
use parser::Parser;
use tokenizer::debug_tokens;
use miette::IntoDiagnostic;

fn main() -> miette::Result<()> {
    loop {
        // prompt
        print!("$ ");
        io::stdout().flush().into_diagnostic()?;
        
        // parse input
        let mut input = String::new();

        io::stdin().read_line(&mut input).into_diagnostic()?;

        // tokenize input
        let _tokens = tokenizer(&input);

        debug_tokens(&_tokens);

        println!("---Debug log ended");

        let mut parser = Parser::new(_tokens);

        let ast = parser.parse().expect("Failed to parse nigger");

        println!("{:#?}", ast);
        // parse input


        // execute read expression through the engine?
        // let mut parts = input.split_whitespace();
        // let command = parts.next().unwrap();
        // let args = parts;

        // let mut child = std::process::Command::new(command)
        //     .args(args)
        //     .spawn()
        //     .into_diagnostic()?;
        // child.wait().into_diagnostic()?;
    }
}
