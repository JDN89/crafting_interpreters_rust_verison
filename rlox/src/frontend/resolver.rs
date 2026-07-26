use std::collections::HashMap;

use crate::frontend::{
    ast::{Expr, Stmt},
};
use anyhow::{bail, Result};

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
}

impl Default for Resolver {
    fn default() -> Self {
        Self { scopes: Vec::new() }
    }
}
impl Resolver {
    fn resolve_statement(&mut self, statement: &Stmt) -> Result<()> {
        match statement {
            Stmt::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
            Stmt::ExpressionStmt { expr } => todo!(),
            Stmt::PrintStmt { expr } => todo!(),
            Stmt::Return { keyword, value } => todo!(),
            Stmt::Var { name, initializer } => {
                self.declare(name);
                if let Some(initializer_expression) = initializer {
                    self.resolve_expression(initializer_expression)?;
                }
                self.define(name);
            }
            Stmt::Block { statements } => {
                self.begin_scope();
                for statement in statements {
                    self.resolve_statement(statement)?;
                }
                self.enc_scope();
            }
            Stmt::While { condition, body } => todo!(),
            Stmt::Function { name, params, body } => todo!(),
        }

        Ok(())
    }

    fn resolve_expression(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Logical { left, op, right } => todo!(),
            Expr::Binary { left, op, right } => todo!(),
            Expr::Call {
                callee,
                paren,
                arguments,
                scope_depth,
            } => todo!(),
            Expr::Assign { name, value } => todo!(),
            Expr::Literal { value } => todo!(),
            Expr::Unary { op, right } => todo!(),
            Expr::Variable { name,scope_depth } => {
                if let Some(scope) = self.scopes.last() {
                    if scope.get(name) == Some(&false) {
                        bail!("Can't read local variable in its own initializer.");
                    }
                }
                // TODO: we store the depth in the AST itself!!
                self.resolve_local(name,scope_depth)
            }
            Expr::Grouping { value } => todo!(),
        }

        Ok(())
    }

    fn enc_scope(&mut self) {
        self.scopes.pop();
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn declare(&mut self, name: &String) {
            if let Some(scope) = self.scopes.last_mut() {
                scope.insert(name.clone(), false);
            }
    }

    fn define(&mut self,name: &String) {
            if let Some(scope) = self.scopes.last_mut() {
                if let Some(value) = scope.get_mut(name) {
                    *value = true
                }
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) -> Result<()> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }

        Ok(())
    }

    fn resolve_local(&self, name: &str, scope_depth: &std::cell::Cell<Option<usize>>) {
        todo!()
    }
}
