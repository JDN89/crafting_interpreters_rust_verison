use rlox::run;

#[test]
fn test_addition() {
    let result = run("1 + 2").unwrap();
    assert_eq!(result, "3");
}

#[test]
fn test_multiplication() {
    let result = run("2 * 3").unwrap();
    assert_eq!(result, "6");
}

#[test]
fn test_operator_precedence() {
    let result = run("1 + 2 * 3").unwrap();
    assert_eq!(result, "7");
}

#[test]
fn test_equality() {
    let result = run("1 == 1").unwrap();
    assert_eq!(result, "true");
}

#[test]
fn test_greater_then() {
    let result = run("3 > 1").unwrap();
    assert_eq!(result, "true");
}
