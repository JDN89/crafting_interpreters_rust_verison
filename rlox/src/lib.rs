pub mod backend;
pub mod frontend;

use anyhow::{Context, Result};
use backend::interpreter::Interpreter;
use frontend::lexer::Lexer;
use frontend::parser::Parser;

pub fn run(source: &str) -> Result<()> {
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().context("Parser error: ")?;
    let interpreter = Interpreter::new();
    interpreter
        .interpret(statements)
        .context("Runtime error: ")?;

    Ok(())
}
