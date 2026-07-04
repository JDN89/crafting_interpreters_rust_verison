use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{Result, bail};

use crate::backend::callable::Clock;
use crate::backend::environment::Env;
use crate::backend::environment::Environment;
use crate::backend::loxfunction::LoxFunction;
use crate::frontend::ast::Stmt;
use crate::frontend::token::TokenType;
use crate::{
    backend::value::LoxValue,
    frontend::ast::{Expr, Operator},
};

pub struct Interpreter {
    pub environment: Env,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl Interpreter {
    pub fn new() -> Self {
        let environment = Environment::new();
        environment
            .borrow_mut()
            .define("clock".to_string(), LoxValue::Callable(Rc::new(Clock)));

        Interpreter {
            // NOTE: globals is env that is accessbile for everyone
            environment,
        }
    }

    fn execute(&mut self, statement: &Stmt) -> Result<()> {
        match statement {
            Stmt::ExpressionStmt { expr } => {
                // Discard result and propagate side effect
                self.evaluate(expr)?;
            }
            Stmt::PrintStmt { expr } => {
                let result = self.evaluate(expr)?;
                // discard result
                println!("{}", result);
            }
            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => LoxValue::Nil,
                };
                self.environment
                    .borrow_mut()
                    .define(name.to_string(), value);
            }
            Stmt::Block { statements } => self.execute_block(
                statements,
                // TODO: change clone to ref?
                Environment::new_enclosed(self.environment.clone()),
            )?,
            Stmt::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(&self.evaluate(condition)?) {
                    self.execute(then_branch)?;
                } else if let Some(stmt) = else_branch {
                    self.execute(stmt)?;
                }
            }
            Stmt::While { condition, body } => {
                while is_truthy(&self.evaluate(condition)?) {
                    let _ = self.execute(body);
                }
            }
            Stmt::Function {name, ..} => {
                let function = LoxFunction {
                      declaration: statement.clone(),
                  };
                self.environment.borrow_mut().define(name.lexeme.clone(), LoxValue::Callable(Rc::new(function)));

            }
            Stmt::Return { value, .. } => {
                if let Some(expr) = value {
                    self.evaluate(expr)?;
                }
            }
        };
        Ok(())
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<()> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<LoxValue> {
        match expr {
            Expr::Binary { left, op, right } => self.evaluate_binary_expression(left, op, right),
            Expr::Assign { name, value } => {
                let evaluated_value = self.evaluate(value)?;
                self.environment
                    .borrow_mut()
                    .assign(name, evaluated_value.clone())?;
                Ok(evaluated_value)
            }
            Expr::Literal { value } => Ok(LoxValue::from(value.clone())),
            Expr::Unary { op, right } => {
                let right = self.evaluate(right)?;
                match op {
                    Operator::Plus => Ok(right),
                    Operator::Minus => negate_value(right),
                    Operator::Bang => Ok(LoxValue::Boolean(!is_truthy(&right))),
                    _ => panic!("Unary should not be possible with the operator types *, / , ="),
                }
            }
            Expr::Variable { name } => self.environment.borrow().get(name),
            Expr::Grouping { value } => self.evaluate(value),
            Expr::Logical { left, op, right } => {
                self.evalutate_logical_expression(left, *op, right)
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evaluate(callee)?;
                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate(arg)?);
                }

                let function = match callee {
                    LoxValue::Callable(function) => function,
                    _ => bail!("Can only call functions and classes."),
                };

                if args.len() != function.arity() {
                    bail!(
                        "Expected {} arguments but got {}.",
                        function.arity(),
                        args.len()
                    );
                }

                let _ = paren;
                function.call(self, args)
            }
        }
    }

    fn evaluate_binary_expression(
        &mut self,
        left: &Expr,
        op: &Operator,
        right: &Expr,
    ) -> Result<LoxValue> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;
        match op {
            Operator::Plus => addition(left, right),
            Operator::Minus => subtraction(left, right),
            Operator::Star => multiplication(left, right),
            Operator::Slash => division(left, right),
            Operator::Equal => {
                bail!("'=' should not appear as an operator in a binary expression",)
            }
            Operator::Bang => {
                bail!("'!' should no appear is the operator in a binary expression",)
            }
            Operator::Greater => greater(left, right),
            Operator::GreaterEqual => greater_then_or_equal(left, right),
            Operator::Less => less(left, right),
            Operator::LessEqual => less_then_or_equal(left, right),
            Operator::EqualEqual => Ok(LoxValue::Boolean(is_equal(left, right))),
            Operator::BangEqual => Ok(LoxValue::Boolean(!is_equal(left, right))),
        }
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        new: Rc<RefCell<Environment>>,
    ) -> Result<()> {
        let previous_env = std::mem::replace(&mut self.environment, new);
          let result = self.interpret(statements);
          self.environment = previous_env;
        //   match result {
        //     Ok(_) => Ok(())
        //     Err(e) => Err(e)
        // }
        // NOTE: discard Ok and return the error value
          result.map(|_| ())
    }

    fn evalutate_logical_expression(
        &mut self,
        left: &Expr,
        op: TokenType,
        right: &Expr,
    ) -> Result<LoxValue> {
        let left = self.evaluate(left)?;

        if op == TokenType::Or {
            if is_truthy(&left) {
                return Ok(left);
            }
        } else {
            // AND branch
            if !is_truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(right)
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
        _ => bail!("can't compare greater than or equal for non-numbers!",),
    }
}
fn less_then_or_equal(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Boolean(l <= r)),
        _ => bail!("can't compare lesser than or equal for non-numbers!",),
    }
}

fn greater(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Boolean(l > r)),
        _ => bail!("can't compare greater than for non-numbers!",),
    }
}
fn less(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Boolean(l < r)),
        _ => bail!("can't compare less than for non-numbers!",),
    }
}

fn addition(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Float(l + r)),
        (LoxValue::Str(l), LoxValue::Str(r)) => Ok(LoxValue::Str(l + &r)),
        _ => bail!("Operands must be two numbers or two strings.",),
    }
}
fn subtraction(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Float(l - r)),
        _ => bail!("can subtract non-numbers!"),
    }
}
fn division(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Float(l / r)),
        _ => bail!("can subtract non-numbers!"),
    }
}
fn multiplication(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Float(l * r)),
        _ => bail!("can subtract non-numbers!"),
    }
}

fn is_truthy(value: &LoxValue) -> bool {
    match value {
        LoxValue::Boolean(b) => *b,
        LoxValue::Nil => false,
        _ => true,
    }
}

fn negate_value(value: LoxValue) -> Result<LoxValue> {
    match value {
        LoxValue::Float(f) => Ok(LoxValue::Float(-f)),
        _ => bail!("Unary applied to a non-number"),
    }
}
