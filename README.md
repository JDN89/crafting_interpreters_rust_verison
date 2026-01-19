# Rlox - rust implementation of Jlox
This is a Rust implementation of the first part of the Crafting Interpreters book, where I attempted to recreate the Java interpreter using Rust. I paused my progress after completing the functions chapter. My project has a bug in recurison that I might fix later, but probably I will rewrite the project later on. Seeing that this was mostly for learning rust and setting up a project that's larger than an Advent of Code challange.

# A fresh start.
Some years ago, I tried to implement a Rust version of jlox and got stuck on recursion. Almost two years later, after returning from a vacation to Peru—where I visited my family-in-law and read part of Programming Rust by Jim Blandy, Jason Orendorff, and Leonora F. S. Tindall—I’ve picked it back up. That earlier failure never really sat well with me. Usually, when I abandon a project, it’s because I get bored. That time was different: I couldn’t find the bug, and after weeks of trying, I gave up and moved on to clox. That sent me down a whole side path of learning C and writing a few toy programs along the way. But now, I’m back.

# Source
[Crafting interpreters by Robert Nystrom](https://craftinginterpreters.com/)

# rLox

read rlox_test.txt that's in the project root: 

```bash

crafting_interpreters_rust_verison/rlox on  master [!?] is 📦 v0.1.0 via 🦀 v1.92.0

❯ cargo run -- rlox_test.txt
   Compiling rlox v0.1.0 (/home/jan/Repos/crafting_interpreters_rust_verison/rlox)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/rlox rlox_test.txt`

run file "rlox_test.txt"
contents: "var a = 1;\n"
```

## Learned

With anyhow crate you can create custom errors on the fly. See error helper function in the code. Downside, you don't have any strongly typed errors and seeing that your error is not defined in an enum, you also can't do any pattern matchin on the error

```Rust
#[derive(Debug)]
enum MyCustomError {
    Marco,
    Polo,
}

// Example of pattern matching on a strongly typed error
fn do_something(e: MyCustomError) {
    match e {
        MyCustomError::Marco => println!("Doing Marco thing!"),
        MyCustomError::Polo => println!("Doing Polo thing!"),
    }
}
```
