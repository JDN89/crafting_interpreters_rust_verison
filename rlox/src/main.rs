use std::env;
use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;

// Custom error reporting helper
// fn error(line: usize, message: &str) -> anyhow::Error {
//     anyhow::anyhow!("[line {}] Error: {}", line, message)
// }

            // return Err(error(i + 1, "Line cannot be empty"));

fn run_file<P>(path:P) -> Result<()> where P: AsRef <Path> {
    let contents = fs::read_to_string(path)
        .context("Should have been able to read the file")?;
    println!("contents: {:?}",contents);
    Ok(())
}

fn main() -> Result<()> {
    let args : Vec<String> = env::args().collect();

    /*  In rust the first argument is the path of the executable.
    In java the first arg is just the first arg
    Usage: rlox [script]
    ["target/debug/rlox", "hello", "you"]
    */

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        println!("{:?}",args)
    }
    else if args.len()== 2 {
        println!("run file {:?}", args[1]);
        run_file(&args[1])?
    }
    else {
        println!("Run Prompt");
    }
    Ok(())
}
