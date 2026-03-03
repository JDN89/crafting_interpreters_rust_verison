use core::panic;

use anyhow::Ok;
use anyhow::Result;

use crate::backend::environment::Environment;
use crate::frontend::ast::Stmt;
use crate::{
    backend::value::LoxValue,
    frontend::ast::{Expr, Operator},
};

pub struct Interpreter {
    environment: Environment,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<()> {
        for stmt in statements {
            match stmt {
                Stmt::ExpressionStmt { expr } => {
                    // Discard result and propagate side effect
                    let _ = self.evaluate(expr)?;
                }

                Stmt::PrintStmt { expr } => {
                    let result = self.evaluate(expr)?;
                    // discard result
                    println!("{}", result);
                }
                Stmt::Var { name, initializer } => {
                    let Some(expr) = initializer else {
                        anyhow::bail!("Cant evaluate a variable without an assigned value!");
                    };
                    let value = self.evaluate(expr)?;
                    self.environment.define(name, value);
                }
            };
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: Expr) -> Result<LoxValue> {
        match expr {
            Expr::Binary { left, op, right } => self.evaluate_binary_expression(*left, op, *right),
            Expr::Assign { op, value } => todo!(),
            Expr::Literal { value } => Ok(LoxValue::from(value)),
            Expr::Unary { op, right } => {
                let right = self.evaluate(*right)?;
                match op {
                    Operator::Plus => Ok(right),
                    Operator::Minus => negate_value(right),
                    Operator::Bang => Ok(is_truthy(right)),
                    _ => panic!("Unary should not be possible with the operator types *, / , ="),
                }
            }
            Expr::Variable { name } => self.environment.get(&name),
            Expr::Grouping { value } => self.evaluate(*value),
        }
        // println!("{:?}", expr);
    }

    fn evaluate_binary_expression(
        &mut self,
        left: Expr,
        op: Operator,
        right: Expr,
    ) -> Result<LoxValue> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;
        match op {
            Operator::Plus => addition(left, right),
            Operator::Minus => subtraction(left, right),
            Operator::Star => multiplication(left, right),
            Operator::Slash => division(left, right),
            Operator::Equal => {
                anyhow::bail!("'=' should not appear as an operator in a binary expression",)
            }
            Operator::Bang => {
                anyhow::bail!("'!' should no appear is the operator in a binary expression",)
            }
            Operator::Greater => greater(left, right),
            Operator::GreaterEqual => greater_then_or_equal(left, right),
            Operator::Less => less(left, right),
            Operator::LessEqual => less_then_or_equal(left, right),
            Operator::EqualEqual => Ok(LoxValue::Boolean(is_equal(left, right))),
            Operator::BangEqual => Ok(LoxValue::Boolean(!is_equal(left, right))),
        }
    }
}

fn is_equal(left: LoxValue, right: LoxValue) -> bool {
    match (left, right) {
        (LoxValue::Nil, LoxValue::Nil) => true,
        (LoxValue::Nil, _) => false,
        (a, b) => a == b,
    }
}

fn greater_then_or_equal(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Boolean(l >= r)),
        _ => anyhow::bail!("can't compare greater than or equal for non-numbers!",),
    }
}
fn less_then_or_equal(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Boolean(l <= r)),
        _ => anyhow::bail!("can't compare lesser than or equal for non-numbers!",),
    }
}

fn greater(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Boolean(l > r)),
        _ => anyhow::bail!("can't compare greater than for non-numbers!",),
    }
}
fn less(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Boolean(l < r)),
        _ => anyhow::bail!("can't compare less than for non-numbers!",),
    }
}

// TODO Return a ()
fn addition(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Float(l + r)),
        (LoxValue::Str(l), LoxValue::Str(r)) => Ok(LoxValue::Str(l + &r)),
        _ => anyhow::bail!("Operands must be two numbers or two strings.",),
    }
}
fn subtraction(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Float(l - r)),
        _ => anyhow::bail!("can subtract non-numbers!"),
    }
}
fn division(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Float(l / r)),
        _ => anyhow::bail!("can subtract non-numbers!"),
    }
}
fn multiplication(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Float(l * r)),
        _ => anyhow::bail!("can subtract non-numbers!"),
    }
}

fn is_truthy(value: LoxValue) -> LoxValue {
    match value {
        LoxValue::Boolean(_) => LoxValue::from(value),
        LoxValue::Nil => LoxValue::Boolean(false),
        _ => LoxValue::Boolean(true),
    }
}

fn negate_value(value: LoxValue) -> Result<LoxValue> {
    match value {
        LoxValue::Float(f) => Ok(LoxValue::Float(-f)),
        _ => anyhow::bail!("Unary applied to a non-number"),
    }
}
