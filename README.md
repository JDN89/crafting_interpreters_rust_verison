# Rlox - rust implementation of Jlox

This is a Rust implementation of the first part of the Crafting Interpreters book, where I attempted to recreate the Java interpreter using Rust. I paused my progress after completing the functions chapter. My project has a bug in recurison that I might fix later, but probably I will rewrite the project later on. Seeing that this was mostly for learning rust and setting up a project that's larger than an Advent of Code challange.

## A fresh start

Some years ago, I tried to implement a Rust version of jlox and got stuck on recursion. Almost two years later, after returning from a vacation to Peru—where I visited my family-in-law and read part of Programming Rust by Jim Blandy, Jason Orendorff, and Leonora F. S. Tindall—I’ve picked it back up. That earlier failure never really sat well with me. Usually, when I abandon a project, it’s because I get bored. That time was different: I couldn’t find the bug, and after weeks of trying, I gave up and moved on to clox. That sent me down a whole side path of learning C and writing a few toy programs along the way. But now, I’m back.

## Source

[Crafting interpreters by Robert Nystrom](https://craftinginterpreters.com/)

## rLox

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

### Running tests

```bash
# Run all tests
cargo test

# Run only lexer tests
cargo test lexer

# Run a specific test
cargo test test_single_left_paren

# Run tests with output
cargo test -- --nocapture
```

### Debugging with rust-gdb

```bash
cargo test --no-run            # build tests without running
rust-gdb target/debug/deps/rlox-<hash>  # start debugger
```

```gdb
# running
run                         # run with last args
run test_name --nocapture   # run single test

# breakpoints
break file.rs:56            # breakpoint at file:line (most reliable)
break rlox::path::Type::fn # breakpoint on Rust method
tbreak file.rs:56           # temporary breakpoint
info breakpoints            # list breakpoints
delete <n>                  # delete breakpoint
disable <n> / enable <n>    # toggle breakpoint

# symbol discovery
info functions name         # find exact Rust symbol

# stepping control
n / next                    # step over line
s / step                    # step into function
finish                      # run until current function returns
until <line>                # run until line in current fn
c / continue                # continue execution

# inspecting state
p expr                      # print expression
info locals                 # show local variables
info args                   # show function arguments
ptype expr                  # show type
bt                           # backtrace / call stack
frame / up / down            # navigate stack frames

# live inspection
display expr                # auto-print expr every stop
info display                 # list displays
undisplay <n>                # remove display
watch expr                  # break when expr changes
rwatch / awatch expr         # break on read / read+write

# source & output
list                        # show source code
set print pretty on          # readable struct output

# rerun & exit
run                          # rerun program
quit                         # exit gdb
```
