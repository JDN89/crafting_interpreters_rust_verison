use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::io::stdout;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;

// use crate::frontend::lexer;
use crate::backend::interpreter::Interpreter;
use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;

mod backend;
mod frontend;

// Custom error reporting helper
// fn error(jline: usize, message: &strj) -> anyhow::Error {
//     anyhow::anyhow!("[line {}] Error: {}", line, message)
// }

// return Err(error(i + 1, "Line cannot be empty"));

fn run_file<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    println!("run file");
    let contents = fs::read_to_string(path).context("Should have been able to read the file")?;
    run(&contents)?;
    Ok(())
}

fn run_prompt() -> Result<()> {
    println!("Running prompt");
    let mut input = String::new();
    loop {
        print!(">  ");
        stdout().flush()?;

        // read_line() appends to input
        input.clear();
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .context("Error readin bytes from stdin")?;
        if bytes_read == 0 {
            break; // EOF
        }

        let input = input.trim_end();
        run(input)?
    }

    Ok(())
}

fn run(source: &str) -> Result<()> {
    let lexer = Lexer::new(source);
    // NOTE: pass ownership of the tokens from the lexer to the parser
    let tokens = lexer.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    // TODO: catch the error and maybe report it?
    // look at the panic mode impl and see how to handle this here
    let expr = parser.parse().context("Parser error: ")?;
    let interpreter = Interpreter::new();
    let result = interpreter.evaluate(expr).context("Runtime error: ")?;
    println!("{}", result);

    // lexer::scan(source);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    /*  In rust the first argument is the path of the executable.
    In java the first arg is just the first arg
    */
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        bail!("Usage: rlox [script]");
    } else if args.len() == 2 {
        println!("run file {:?}", args[1]);
        run_file(&args[1])?
    } else {
        run_prompt()?
    }
    Ok(())
}
