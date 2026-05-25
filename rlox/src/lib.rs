
use anyhow::{Context, Result};

pub mod frontend;
pub mod backend;

use crate::backend::interpreter::Interpreter;
use crate::backend::value::LoxValue;

use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;

pub fn run(source: &str, interpreter: &mut Interpreter) -> Result<LoxValue> {
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
    let result = interpreter
        .interpret(&statements)
        .context("Runtime error: ")?;

    println!("interpreted result {:?}", result);

    Ok(result)
}
