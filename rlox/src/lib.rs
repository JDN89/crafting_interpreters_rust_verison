use anyhow::Result;

pub mod backend;
pub mod frontend;

use crate::backend::interpreter::Interpreter;
use crate::backend::value::LoxValue;

use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;

fn eval_internal(source: &str, interpreter: &mut Interpreter) -> Result<LoxValue> {
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    //NOTE ran into a bug where the environemnt inside interpreter got cleared during running of the
    //repl with each new line. the issues was that run is owning and recreating the environment with
    //each function call. solution, make run prompt create the interpreter and pass it here. leave here
    //for now for educational purposes
    // let mut interpreter = Interpreter::new();
    interpreter.interpret(&statements)
}

pub fn run(source: &str, interpreter: &mut Interpreter) -> Result<LoxValue> {
    let result = eval_internal(source, interpreter)?;

    println!("interpreted result {:?}", result);

    Ok(result)
}

pub fn test_eval(source: &str) -> Result<LoxValue> {
    let mut interpreter = Interpreter::new();
    eval_internal(source, &mut interpreter)
}
