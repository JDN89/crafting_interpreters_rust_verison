pub mod backend;
pub mod frontend;

use anyhow::{Context, Result};
use backend::interpreter::Interpreter;
use frontend::lexer::Lexer;
use frontend::parser::Parser;

pub fn run(source: &str, interpreter: &mut Interpreter) -> Result<()> {
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens()?;
    // for token in &tokens {
    //     println!("{}", &token);
    // }
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().context("Parser error: ")?;
    //
    //NOTE ran into a bug where the environemnt inside interpreter got cleared during running of the
    //repl with each new line. the issues was that run is owning and recreating the environment with
    //each function call. solution, make run prompt create the interpreter and pass it here. leave here
    //for now for educational purposes
    // let mut interpreter = Interpreter::new();
    interpreter
        .interpret(statements)
        .context("Runtime error: ")?;

    Ok(())
}
