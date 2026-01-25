use std::env;
use std::fs;
use std::io;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;

// use crate::frontend::lexer;
use crate::frontend::lexer::Lexer;

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
        let bytes_read = io::stdin().read_line(&mut input)?;
        if bytes_read == 0 {
            break; // EOF
        }

        let input = input.trim_end();
        run(input)?
    }

    Ok(())
}

fn run(source: &str) -> Result<()> {
    println!("the source is : {:?}", source);
    let mut lexer = Lexer::new(source);
    let _ = lexer.scan_tokens();

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
