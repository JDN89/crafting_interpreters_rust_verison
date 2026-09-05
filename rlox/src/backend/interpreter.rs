use std::rc::Rc;

use anyhow::{Result, bail};

use crate::backend::callable::Clock;
use crate::backend::environment::Env;
use crate::backend::environment::Environment;
use crate::backend::environment::GlobalEnvironment;
use crate::backend::exec_signal::ExecSignal;
use crate::backend::loxfunction::LoxFunction;
use crate::frontend::ast::Stmt;
use crate::frontend::token::TokenType;
use crate::{
    backend::value::LoxValue,
    frontend::ast::{Expr, Operator},
};

pub struct Interpreter {
    //TODO globals should be a sep type that has hashmap and enclosing ENV
    //
    pub globals: GlobalEnvironment,
    pub environment: Env,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    #[must_use]
    pub fn new() -> Self {
        let mut globals = GlobalEnvironment::new();
        globals.define_global_value("clock".to_string(), LoxValue::Callable(Rc::new(Clock)));

        Self {
            // NOTE: globals is env that is accessbile for everyone
            globals: globals,
            //BUG: possibility of bug at this stage. We used to clone globals for our local environment
            environment: Environment::new(),
        }
    }

    fn execute_statement(&mut self, statement: &Stmt) -> Result<ExecSignal> {
        match statement {
            Stmt::ExpressionStmt { expr } => {
                // Discard result and propagate side effect
                self.evaluate_expression(expr)?;
                return Ok(ExecSignal::Normal);
            }
            Stmt::PrintStmt { expr } => {
                let result = self.evaluate_expression(expr)?;
                // discard result
                println!("{result}");
                return Ok(ExecSignal::Normal);
            }
            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.evaluate_expression(expr)?,
                    None => LoxValue::Nil,
                };
                //TODO var a = 1-> ath this moment define in environment.
                // becasue these are the values we want to retrieve that are bound to a scope -> var a = global { var a = local; print a;} print a;
                // SAME for Function names and arguments
                self.environment.borrow_mut().define(name.clone(), value);
                return Ok(ExecSignal::Normal);
            }
            Stmt::Block { statements } => {
                return self.execute_block(
                    statements,
                    Environment::new_enclosed(self.environment.clone()),
                );
            }
            Stmt::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(&self.evaluate_expression(condition)?) {
                    return self.execute_statement(then_branch);
                } else if let Some(stmt) = else_branch {
                    return self.execute_statement(stmt);
                }

                return Ok(ExecSignal::Normal);
            }
            Stmt::While { condition, body } => {
                while is_truthy(&self.evaluate_expression(condition)?) {
                    match self.execute_statement(body)? {
                        ExecSignal::Normal => {}
                        signal @ ExecSignal::Return(_) => return Ok(signal),
                    }
                }
            }
            //TODO var a = 1-> ath this moment define in environment.
            // becasue these are the values we want to retrieve that are bound to a scope -> var a = global { var a = local; print a;} print a;
            // SAME for Function names and arguments
            Stmt::Function { name, .. } => {
                let function = LoxFunction {
                    declaration: statement.clone(),
                    closure: self.environment.clone(),
                };
                //TODO funciton name we store in environment
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), LoxValue::Callable(Rc::new(function)));
            }
            Stmt::Return { value, .. } => {
                let value = match value {
                    Some(expr) => self.evaluate_expression(expr)?,
                    None => LoxValue::Nil,
                };
                return Ok(ExecSignal::Return(value));
            }
        }
        Ok(ExecSignal::Normal)
    }

    fn execute_statements(&mut self, statements: &Vec<Stmt>) -> Result<ExecSignal> {
        for stmt in statements {
            match self.execute_statement(stmt)? {
                ExecSignal::Normal => {}
                signal @ ExecSignal::Return(_) => return Ok(signal),
            }
        }

        Ok(ExecSignal::Normal)
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<()> {
        match self.execute_statements(statements)? {
            ExecSignal::Normal => Ok(()),
            ExecSignal::Return(_) => bail!("Can't return from top-level code."),
        }
    }

    fn evaluate_expression(&mut self, expr: &Expr) -> Result<LoxValue> {
        match expr {
            Expr::Binary { left, op, right } => self.evaluate_binary_expression(left, *op, right),
            Expr::Assign {
                name,
                value,
                env_location,
            } => {
                let evaluated_value = self.evaluate_expression(value)?;

                //TODO During the resolving face we figure out at whcih depth we have to store a Assingment Expr.
                // So during interpretation we use the scope depth to determine where to store the value.
                // so for Literals we define at whcih depth the value will be stored and in which slot
                // I am wrong, the scope hashmap won't match the interpreter's environment
                // I am still missing somehting. go further tomorrow.
                // Is it as simple as adding the slot and Deth to the LoxValue itslef?
                match env_location.get() {
                    Some((depth, slot)) => Environment::assign_at(
                        &self.environment,
                        depth,
                        slot,
                        name,
                        evaluated_value.clone(),
                    )?,
                    None => self
                        .globals
                        .assign_global_value(name, evaluated_value.clone())?,
                }

                Ok(evaluated_value)
            }
            Expr::Literal { value } => Ok(LoxValue::from(value.clone())),
            Expr::Unary { op, right } => {
                let right = self.evaluate_expression(right)?;
                match op {
                    Operator::Plus => Ok(right),
                    Operator::Minus => negate_value(&right),
                    Operator::Bang => Ok(LoxValue::Boolean(!is_truthy(&right))),
                    _ => bail!("Unary should not be possible with the operator types *, / , ="),
                }
            }

            // TODO: during resolving when it's global we don't give a depth?
            Expr::Variable { name, env_location } => match env_location.get() {
                // TODO If there is a depth, get the value from the local scope otherwise get from globals
                // THIS is the CRUX look furhter tomorrow
                // what I am missing is that at this point the Expr::Variable is allready stored in the environement??
                // I think this is wrong we only have a dept for Expr::Assignment, because it's fot the litereal
                Some((depth, slot)) => Environment::get_at(&self.environment, depth, slot, name),
                None => self.globals.get_global_value(name),
            },
            Expr::Grouping { value } => self.evaluate_expression(value),
            Expr::Logical { left, op, right } => {
                self.evalutate_logical_expression(left, *op, right)
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evaluate_expression(callee)?;
                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate_expression(arg)?);
                }

                let LoxValue::Callable(function) = callee else {
                    bail!("Can only call functions and classes.")
                };

                if args.len() != function.arity() {
                    bail!(
                        "Expected {} arguments but got {}.",
                        function.arity(),
                        args.len()
                    );
                }

                let _ = paren;
                // LoxCallable -> native and non-native functions get evaluated at this point.
                // the function  gets evaluated and returns a value in case of a return value, in the other case we return LoxValue::NIL
                function.call(self, args)
            }
        }
    }

    fn evaluate_binary_expression(
        &mut self,
        left: &Expr,
        op: Operator,
        right: &Expr,
    ) -> Result<LoxValue> {
        let left = self.evaluate_expression(left)?;
        let right = self.evaluate_expression(right)?;
        match op {
            Operator::Plus => addition(left, right),
            Operator::Minus => subtraction(left, right),
            Operator::Star => multiplication(left, right),
            Operator::Slash => division(left, right),
            Operator::Equal => {
                bail!("'=' should not appear as an operator in a binary expression")
            }
            Operator::Bang => {
                bail!("'!' should no appear is the operator in a binary expression")
            }
            Operator::Greater => greater(left, right),
            Operator::GreaterEqual => greater_then_or_equal(left, right),
            Operator::Less => less(left, right),
            Operator::LessEqual => less_then_or_equal(left, right),
            Operator::EqualEqual => Ok(LoxValue::Boolean(is_equal(left, right))),
            Operator::BangEqual => Ok(LoxValue::Boolean(!is_equal(left, right))),
        }
    }

    pub fn execute_block(&mut self, statements: &Vec<Stmt>, new: Env) -> Result<ExecSignal> {
        let previous_env = std::mem::replace(&mut self.environment, new);
        let result = self.execute_statements(statements);
        self.environment = previous_env;
        result
    }

    fn evalutate_logical_expression(
        &mut self,
        left: &Expr,
        op: TokenType,
        right: &Expr,
    ) -> Result<LoxValue> {
        let left = self.evaluate_expression(left)?;

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

        self.evaluate_expression(right)
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
        _ => bail!("can't compare greater than or equal for non-numbers!"),
    }
}
fn less_then_or_equal(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Boolean(l <= r)),
        _ => bail!("can't compare lesser than or equal for non-numbers!"),
    }
}

fn greater(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Boolean(l > r)),
        _ => bail!("can't compare greater than for non-numbers!"),
    }
}
fn less(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Boolean(l < r)),
        _ => bail!("can't compare less than for non-numbers!"),
    }
}

fn addition(left: LoxValue, right: LoxValue) -> Result<LoxValue> {
    match (left, right) {
        (LoxValue::Float(l), LoxValue::Float(r)) => Ok(LoxValue::Float(l + r)),
        (LoxValue::Str(l), LoxValue::Str(r)) => Ok(LoxValue::Str(l + &r)),
        _ => bail!("Operands must be two numbers or two strings."),
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

const fn is_truthy(value: &LoxValue) -> bool {
    match value {
        LoxValue::Boolean(b) => *b,
        LoxValue::Nil => false,
        _ => true,
    }
}

fn negate_value(value: &LoxValue) -> Result<LoxValue> {
    match value {
        LoxValue::Float(f) => Ok(LoxValue::Float(-f)),
        _ => bail!("Unary applied to a non-number"),
    }
}
