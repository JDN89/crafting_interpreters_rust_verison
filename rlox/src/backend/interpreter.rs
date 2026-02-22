use core::panic;

use crate::frontend::ast::{Expr, Literal, Operator};

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
    pub fn evaluate(&self, expr: Expr) -> Literal {
        match expr {
            Expr::Binary { left, op, right } => self.evaluate_binary_expression(*left, op, *right),
            Expr::Assign { op, value } => todo!(),
            Expr::Literal { value } => value,
            Expr::Unary { op, right } => {
                let right = self.evaluate(*right);
                match op {
                    Operator::Plus => right,
                    Operator::Minus => negate_literal(right),
                    Operator::Bang => is_truthy(right),
                    _ => panic!("Unary should not be possible with the operator types *, / , ="),
                }
            }
            Expr::Variable { name } => todo!(),
            Expr::Grouping { value } => self.evaluate(*value),
        }
        // println!("{:?}", expr);
    }

    fn evaluate_binary_expression(&self, left: Expr, op: Operator, right: Expr) -> Literal {
        let left = self.evaluate(left);
        let right = self.evaluate(right);
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
            Operator::BangEqual => is_bang_equal(left, right),
            Operator::EqualEqual => is_equal(left, right),
        }
    }
}

// TODO : rewrite this week. Not clear at all
fn is_bang_equal(left: Literal, right: Literal) -> Literal {
    match (left, right) {
        (Literal::Nil, Literal::Nil) => Literal::Boolean(false),
        (Literal::Nil, _) => Literal::Boolean(true),
        (a, b) => Literal::Boolean(a != b),
    }
}

fn is_equal(left: Literal, right: Literal) -> Literal {
    match (left, right) {
        (Literal::Nil, Literal::Nil) => Literal::Boolean(true),
        (Literal::Nil, _) => Literal::Boolean(false),
        (a, b) => Literal::Boolean(a == b),
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

fn addition(left: Literal, right: Literal) -> Literal {
    match (left, right) {
        (Literal::Float(l), Literal::Float(r)) => Literal::Float(l + r),
        (Literal::Str(l), Literal::Str(r)) => Literal::Str(l + &r),
        _ => panic!("can subtract non-numbers!"),
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
fn multiplication(left: Literal, right: Literal) -> Literal {
    match (left, right) {
        (Literal::Float(l), Literal::Float(r)) => Literal::Float(l * r),
        _ => panic!("can subtract non-numbers!"),
    }
}

fn is_truthy(literal: Literal) -> Literal {
    match literal {
        Literal::Boolean(_) => literal,
        Literal::Nil => Literal::Boolean(false),
        _ => Literal::Boolean(true),
    }
}

fn negate_literal(lit: Literal) -> Literal {
    match lit {
        Literal::Float(f) => Literal::Float(-f),
        _ => panic!("Unary applied to a non-number"),
    }
}
