pub mod backend;
pub mod frontend;

use anyhow::{Context, Result};
use backend::interpreter::Interpreter;
use frontend::lexer::Lexer;
use frontend::parser::Parser;

pub fn run(source: &str) -> Result<String> {
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().context("Parser error: ")?;
    let interpreter = Interpreter::new();
    let result = interpreter.evaluate(expr).context("Runtime error: ")?;
    println!("{}", result);

    Ok(result.to_string())
}
