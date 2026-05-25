use rlox::{backend::{interpreter::Interpreter, value::LoxValue}, run};

#[test]
fn test_addition() -> anyhow::Result<()> {
    let mut interpreter = setup_integation_test();

    let input = "1+2;";
    let result = run(input, &mut interpreter)?;

    assert_eq!(result, LoxValue::Float(3.0));

    Ok(())
}


fn setup_integation_test() -> Interpreter {
    let interpreter = Interpreter::new();
    interpreter
}
