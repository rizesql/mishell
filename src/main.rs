use std::io::{self, Write};

use miette::IntoDiagnostic;

fn main() -> miette::Result<()> {
    loop {
        // prompt
        print!("$ ");
        io::stdout().flush().into_diagnostic()?;

        // parse input
        let mut input = String::new();
        io::stdin().read_line(&mut input).into_diagnostic()?;

        // execute read expression through the engine?
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        let mut child = std::process::Command::new(command)
            .args(args)
            .spawn()
            .into_diagnostic()?;
        child.wait().into_diagnostic()?;
    }
}
