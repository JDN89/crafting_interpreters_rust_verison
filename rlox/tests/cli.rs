use std::io::Write;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;

fn run_script(source: &str) -> assert_cmd::assert::Assert {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(source.as_bytes()).unwrap();
    file.flush().unwrap();

    let mut cmd = Command::cargo_bin("rlox").unwrap();
    cmd.arg(file.path());
    cmd.assert()
}

#[test]
fn prints_arithmetic_results() {
    run_script("print 1 + 2;")
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn prints_concatenated_strings() {
    run_script("print \"hi\" + \" there\";")
        .success()
        .stdout(predicate::str::contains("hi there"));
}

#[test]
fn keeps_variable_assignments() {
    run_script("var a = 1; a = a + 2; print a;")
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn respects_block_scopes() {
    run_script("var a = \"outer\"; { var a = \"inner\"; print a; } print a;")
        .success()
        .stdout(predicate::str::contains("inner").and(predicate::str::contains("outer")));
}

#[test]
fn reports_runtime_type_errors() {
    run_script("print \"a\" - \"b\";")
        .failure()
        .stderr(predicate::str::contains("can subtract non-numbers!"));
}

#[test]
fn fib_test() {
    let source = "
    fun fib(n) {
      if (n <= 1) return n;
      return fib(n - 2) + fib(n - 1);
    }

    for (var i = 0; i < 20; i = i + 1) {
      print fib(i);
    }
    ";
    run_script(source)
        .success()
        .stdout(predicate::str::contains("4181"));
}

#[test]
fn closures_test() {

    let source = "
        fun makeCounter() {
          var i = 0;
          fun count() {
            i = i + 1;
            print i;
          }

          return count;
        }

        var counter = makeCounter();
        counter();
        counter();
        ";
    run_script(source)
        .success()
        .stdout(predicate::str::contains("1")
        .and(predicate::str::contains("2")));
}

#[test]
fn static_scope_test() {
    let source = include_str!("static_scope_bug.lox");
    run_script(source)
        .success()
        .stdout(predicate::str::contains("global\nglobal"));
}
