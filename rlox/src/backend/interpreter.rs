use core::panic;

use crate::{
    backend::value::{RuntimeError, Value},
    frontend::ast::{Expr, Literal, Operator},
};

pub struct Interpreter {}

#[allow(dead_code)]
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    // TODO: replace panic once done
    // NOTE maybe compute literal in the itnerpreter face and just pass the start and end of the
    // source code to the interpreter?
    // NOTE reuse Literal becuase it contains the literal values...
    pub fn evaluate(&self, expr: Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Binary { left, op, right } => self.evaluate_binary_expression(*left, op, *right),
            Expr::Assign { op, value } => todo!(),
            Expr::Literal { value } => Ok(Value::from(value)),
            Expr::Unary { op, right } => {
                let right = self.evaluate(*right)?;
                match op {
                    Operator::Plus => Ok(right),
                    Operator::Minus => negate_value(right),
                    Operator::Bang => is_truthy(right),
                    _ => panic!("Unary should not be possible with the operator types *, / , ="),
                }
            }
            Expr::Variable { name } => todo!(),
            Expr::Grouping { value } => self.evaluate(*value),
        }
        // println!("{:?}", expr);
    }

    fn evaluate_binary_expression(
        &self,
        left: Expr,
        op: Operator,
        right: Expr,
    ) -> Result<Value, RuntimeError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;
        match op {
            Operator::Plus => addition(left, right),
            Operator::Minus => subtraction(left, right),
            Operator::Star => multiplication(left, right),
            Operator::Slash => division(left, right),
            Operator::Equals => {
                panic!("'=' should not appear as the operator in a binary expression")
            }
            Operator::Bang => panic!("'!' should no appear is the operator in a binary expression"),
            Operator::Greater => greater(left, right),
            Operator::GreaterEqual => greater_then_or_equal(left, right),
            Operator::Less => less(left, right),
            Operator::LessEqual => less_then_or_equal(left, right),
            Operator::EqualEqual => Literal::Boolean(is_equal(left, right)),
            Operator::BangEqual => Literal::Boolean(!is_equal(left, right)),
        }
    }
}

fn is_equal(left: Literal, right: Literal) -> bool {
    match (left, right) {
        (Literal::Nil, Literal::Nil) => true,
        (Literal::Nil, _) => false,
        (a, b) => a == b,
    }
}

fn greater_then_or_equal(left: Literal, right: Literal) -> Literal {
    match (left, right) {
        (Literal::Float(l), Literal::Float(r)) => Literal::Boolean(l >= r),
        _ => panic!("can't compare greater than or equal for non-numbers!"),
    }
}
fn less_then_or_equal(left: Literal, right: Literal) -> Literal {
    match (left, right) {
        (Literal::Float(l), Literal::Float(r)) => Literal::Boolean(l <= r),
        _ => panic!("can't compare lesser than or equal for non-numbers!"),
    }
}

fn greater(left: Literal, right: Literal) -> Literal {
    match (left, right) {
        (Literal::Float(l), Literal::Float(r)) => Literal::Boolean(l > r),
        _ => panic!("can't compare greater than for non-numbers!"),
    }
}
fn less(left: Literal, right: Literal) -> Literal {
    match (left, right) {
        (Literal::Float(l), Literal::Float(r)) => Literal::Boolean(l < r),
        _ => panic!("can't compare less than for non-numbers!"),
    }
}

// TODO Return a RuntimeError
fn addition(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (left, right) {
        (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l + r)),
        (Value::Str(l), Value::Str(r)) => Ok(Value::Str(l + &r)),
        _ => Err(RuntimeError::new(
            "Operands must be two numbers or two strings.",
        )),
    }
}
fn subtraction(left: Literal, right: Literal) -> Literal {
    match (left, right) {
        (Literal::Float(l), Literal::Float(r)) => Literal::Float(l - r),
        _ => panic!("can subtract non-numbers!"),
    }
}
fn division(left: Literal, right: Literal) -> Literal {
    match (left, right) {
        (Literal::Float(l), Literal::Float(r)) => Literal::Float(l / r),
        _ => panic!("can subtract non-numbers!"),
    }
}
fn multiplication(left: Value, right: Value) -> Result<Value, RuntimeError> {
    match (left, right) {
        (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l * r)),
        _ => Err(RuntimeError::new("can subtract non-numbers!")),
    }
}

fn is_truthy(value: Value) -> Value {
    match value {
        Value::Boolean(_) => Value::from(value),
        Value::Nil => Value::Boolean(false),
        _ => Value::Boolean(true),
    }
}

fn negate_value(value: Value) -> Result<Value, RuntimeError> {
    match value {
        Value::Float(f) => Ok(Value::Float(-f)),
        _ => Err(RuntimeError::new("Unary applied to a non-number")),
    }
}
