use rlox::{
    backend::value::LoxValue,
    test_eval,
};

#[test]
fn test_addition() -> anyhow::Result<()> {
    let input = "1+2;";
    let result = test_eval(input)?;

    assert_eq!(result, LoxValue::Float(3.0));

    Ok(())
}

#[test]
fn test_clock_call() -> anyhow::Result<()> {
    let result = test_eval("clock();")?;

    match result {
        LoxValue::Float(n) => assert!(n > 0.0),
        other => panic!("expected float result, got {other:?}"),
    }

    Ok(())
}

#[test]
fn test_call_arity_error() {
    let result = test_eval("clock(1);");

    assert!(result.is_err());
}

#[test]
fn test_while_loop() -> anyhow::Result<()> {
    let input = "
         var i = 0;
         while (i < 10) {
           print i;
           i = i + 1;
         }
       ";
    let result = test_eval(input)?;

    assert_eq!(result, LoxValue::Nil);
    Ok(())
}

#[test]
fn test_for_loop() -> anyhow::Result<()> {
    let input = "
       for (var i = 0; i < 10; i = i + 1) print i;
       ";
    let result = test_eval(input)?;

    assert_eq!(result, LoxValue::Nil);
    Ok(())
}
#[test]
fn is_truthy_check() -> anyhow::Result<()> {
    let input = r#"
        "hi" or 2;
        nil or "yes";
    "#;

    let result = test_eval(input)?;

    assert_eq!(result, LoxValue::Str("\"yes\"".to_string()));

    Ok(())
}

#[test]
fn test_complext_for_loop() -> anyhow::Result<()> {
    let input = "
var a = 0;
var temp;

for (var b = 1; a < 10000; b = temp + b) {
  print a;
  temp = a;
  a = b;
}
        ";

    let result = test_eval(input)?;

    assert_eq!(result, LoxValue::Nil);

    Ok(())
}
