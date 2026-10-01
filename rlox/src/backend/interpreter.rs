use std::rc::Rc;

use anyhow::Ok;
use anyhow::{Result, bail};

use crate::backend::callable::Clock;
use crate::backend::environment::Env;
use crate::backend::environment::Environment;
use crate::backend::environment::GlobalEnvironment;
use crate::backend::exec_signal::ExecSignal;
use crate::backend::loxfunction::LoxFunction;
use crate::frontend::ast::Ast;
use crate::frontend::ast::ExprId;
use crate::frontend::ast::Stmt;
use crate::frontend::ast::StmtId;
use crate::frontend::token::TokenType;
use crate::{
    backend::value::LoxValue,
    frontend::ast::{Expr, Operator},
};

pub struct Interpreter {
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
            globals,
            environment: Environment::new(),
        }
    }

    fn execute_statement(&mut self, statement: &Stmt, ast: &Ast) -> Result<ExecSignal> {
        match statement {
            Stmt::ExpressionStmt { expr: expr_id } => {
                // Discard result and propagate side effect
                // TODO check is this the correct pattern?
                let _result = self.evaluate_expression(ast.get_expression(*expr_id)?, ast);
                return Ok(ExecSignal::Normal);
            }
            Stmt::PrintStmt { expr } => {
                let result = self.evaluate_expression_by_id(ast, expr)?;
                // discard result
                println!("{result}");
                return Ok(ExecSignal::Normal);
            }
            Stmt::Var {
                name,
                initializer,
                env_location,
            } => {
                let value = match initializer {
                    Some(expr) => self.evaluate_expression_by_id(ast, expr)?,
                    None => LoxValue::Nil,
                };
                match env_location.get() {
                    Some((_depth, _slot)) => self.environment.borrow_mut().define(value),
                    None => self.globals.define_global_value(name.clone(), value),
                }
                return Ok(ExecSignal::Normal);
            }
            Stmt::Block { statements } => {
                return self.execute_block(
                    Environment::new_enclosed(self.environment.clone()),
                    statements,
                    ast,
                );
            }
            Stmt::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(&self.evaluate_expression(ast.get_expression(*condition)?, ast)?) {
                    return self.execute_statement(ast.get_statement(*then_branch)?, ast);
                } else if let Some(stmt) = else_branch {
                    return self.execute_statement(ast.get_statement(*stmt)?, ast);
                }

                return Ok(ExecSignal::Normal);
            }
            Stmt::While { condition, body } => {
                while is_truthy(&self.evaluate_expression(ast.get_expression(*condition)?, ast)?) {
                    match self.execute_statement(ast.get_statement(*body)?, ast)? {
                        ExecSignal::Normal => {}
                        signal @ ExecSignal::Return(_) => return Ok(signal),
                    }
                }
            }

            Stmt::Function {
                name,
                params: _params,
                body: _body,
                env_location,
            } => {
                let function = LoxFunction {
                    declaration: statement.clone(),
                    closure: self.environment.clone(),
                };
                // RESEARCH: for define is slot and depth needed?

                match env_location.get() {
                    Some(_) => self
                        .environment
                        .borrow_mut()
                        .define(LoxValue::Callable(Rc::new(function))),
                    None => self.globals.define_global_value(
                        name.lexeme.clone(),
                        LoxValue::Callable(Rc::new(function)),
                    ),
                }
            }
            Stmt::Return { value, .. } => {
                let value = match value {
                    Some(expr) => self.evaluate_expression_by_id(ast, expr)?,
                    None => LoxValue::Nil,
                };
                return Ok(ExecSignal::Return(value));
            }
        }
        Ok(ExecSignal::Normal)
    }

    // TODO: continue implementing this. I didn't use this helper function everywhere...
    fn evaluate_expression_by_id(
        &mut self,
        ast: &Ast,
        expr: &ExprId,
    ) -> Result<LoxValue, anyhow::Error> {
        Ok(self.evaluate_expression(ast.get_expression(*expr)?, ast)?)
    }

    fn execute_ast_root_node_statements(&mut self, ast: &Ast) -> Result<ExecSignal> {
        for stmt in &ast.ast_root_nodes {
            match self.execute_statement(&stmt, ast)? {
                ExecSignal::Normal => {}
                signal @ ExecSignal::Return(_) => return Ok(signal),
            }
        }

        Ok(ExecSignal::Normal)
    }

    pub fn interpret(&mut self, ast: &Ast) -> Result<()> {
        match self.execute_ast_root_node_statements(ast)? {
            ExecSignal::Normal => Ok(()),
            ExecSignal::Return(_) => bail!("Can't return from top-level code."),
        }
    }

    fn evaluate_expression(&mut self, expr: &Expr, ast: &Ast) -> Result<LoxValue> {
        match expr {
            Expr::Binary { left, op, right } => self.evaluate_binary_expression(
                ast.get_expression(*left)?,
                *op,
                ast.get_expression(*right)?,
                ast,
            ),
            Expr::Assign {
                name,
                value,
                env_location,
            } => {
                let evaluated_value = self.evaluate_expression(ast.get_expression(*value)?, ast)?;

                // TODO: look at how to remove.clone
                match env_location.get() {
                    Some((depth, slot)) => Environment::assign_at(
                        &self.environment,
                        depth,
                        slot,
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
                let right = self.evaluate_expression(ast.get_expression(*right)?, ast)?;
                match op {
                    Operator::Plus => Ok(right),
                    Operator::Minus => negate_value(&right),
                    Operator::Bang => Ok(LoxValue::Boolean(!is_truthy(&right))),
                    _ => bail!("Unary should not be possible with the operator types *, / , ="),
                }
            }

            Expr::Variable { name, env_location } => match env_location.get() {
                Some((depth, slot)) => Environment::get_at(&self.environment, depth, slot, name),
                None => self.globals.get_global_value(name),
            },
            Expr::Grouping { value } => self.evaluate_expression(ast.get_expression(*value)?, ast),
            Expr::Logical { left, op, right } => self.evalutate_logical_expression(
                ast.get_expression(*left)?,
                *op,
                ast.get_expression(*right)?,
                ast,
            ),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evaluate_expression(ast.get_expression(*callee)?, ast)?;
                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate_expression(ast.get_expression(*arg)?, ast)?);
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
                function.call(self, args, ast)
            }
        }
    }

    fn evaluate_binary_expression(
        &mut self,
        left: &Expr,
        op: Operator,
        right: &Expr,
        ast: &Ast,
    ) -> Result<LoxValue> {
        let left = self.evaluate_expression(left, ast)?;
        let right = self.evaluate_expression(right, ast)?;
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

    pub fn execute_block(
        &mut self,
        new: Env,
        statement_ids: &[StmtId],
        ast: &Ast,
    ) -> Result<ExecSignal> {
        let previous_env = std::mem::replace(&mut self.environment, new);

        // NOTE: sort of the rust equivalent of a finally in Java
        // closure here, it lets us restore the environment after '?' or 'return' esits early.
        let result = (|| {
            for statement_id in statement_ids {
                match self.execute_statement(ast.get_statement(*statement_id)?, ast)? {
                    ExecSignal::Normal => {}
                    signal @ ExecSignal::Return(_) => return Ok(signal),
                }
            }
            Ok(ExecSignal::Normal)
        })();

        // closure (finally) lets us always restore the environment.
        self.environment = previous_env;
        result
    }

    fn evalutate_logical_expression(
        &mut self,
        left: &Expr,
        op: TokenType,
        right: &Expr,
        ast: &Ast,
    ) -> Result<LoxValue> {
        let left = self.evaluate_expression(left, ast)?;

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

        self.evaluate_expression(right, ast)
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
