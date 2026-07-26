use std::collections::HashMap;

use crate::frontend::{
    ast::{Expr, Stmt},
    token::Token,
};
use anyhow::Result;

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
}

impl Default for Resolver {
    fn default() -> Self {
        Self { scopes: Vec::new() }
    }
}
impl Resolver {
    fn resolve_statement(&mut self, statement: &Stmt) {
        match statement {
            Stmt::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
            Stmt::ExpressionStmt { expr } => todo!(),
            Stmt::PrintStmt { expr } => todo!(),
            Stmt::Return { keyword, value } => todo!(),
            Stmt::Var { name, initializer } => todo!(),
            Stmt::Block { statements } => todo!(),
            Stmt::While { condition, body } => todo!(),
            Stmt::Function { name, params, body } => todo!(),
        }
    }

    fn resolve_expression(&mut self, expr: &Expr) {
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
            Expr::Variable { name } => todo!(),
            Expr::Grouping { value } => todo!(),
        }
    }

    fn enc_scope(&mut self) {
        self.scopes.pop();
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn declare(&mut self, name: &Token) {
        if !self.scopes.is_empty() {
            if let Some(scope) = self.scopes.last_mut() {
                scope.insert(name.lexeme.clone(), false);
            }
        }
    }

    fn define(&mut self,token: &Token) {
        if!self.scopes.is_empty() {
            if let Some(scope) = self.scopes.last_mut() {
                if let Some(value) = scope.get_mut(&token.lexeme) {
                    *value = true
                }
            }
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) -> Result<()> {
        for statement in statements {
            self.resolve_statement(statement);
        }

        Ok(())
    }
}
