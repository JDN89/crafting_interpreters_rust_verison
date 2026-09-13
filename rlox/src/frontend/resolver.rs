use std::{cell::Cell, collections::HashMap};

use crate::frontend::ast::{Ast, Depth, Expr, ExprId, Slot, Stmt, StmtId};
use anyhow::{Result, anyhow, bail};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FunctionType {
    None,
    Function,
}

pub struct Resolver {
    scopes: Vec<HashMap<String, (Slot, bool)>>,
    current_function: FunctionType,
}

impl Default for Resolver {
    fn default() -> Self {
        Self {
            scopes: Vec::new(),
            current_function: FunctionType::None,
        }
    }
}
impl Resolver {
    // ----------------HELPER functions ----------------
    fn resolve_expression_id(&mut self, id: ExprId, ast: &Ast) -> Result<()> {
        let expression = ast
            .get_expression(id)
            .ok_or_else(|| anyhow!("Invalid expression ID : {id}"))?;
        self.resolve_expression(expression, ast);
        Ok(())
    }

    fn resolve_statement_id(&mut self, id: StmtId, ast: &Ast) -> Result<()> {
        let statement = ast
            .get_statement(id)
            .ok_or_else(|| anyhow!("Invalid statement ID : {id}"))?;
        self.resolve_statement(statement, ast);
        Ok(())
    }

    // ----------------------------

    fn resolve_statement(&mut self, statement: &Stmt, ast: &Ast) -> Result<()> {
        match statement {
            Stmt::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expression_id(*condition, ast)?;
                self.resolve_statement_id(*then_branch, ast)?;
                if let Some(else_stmt) = else_branch {
                    self.resolve_statement_id(*else_stmt, ast)?;
                }
            }
            Stmt::ExpressionStmt { expr } | Stmt::PrintStmt { expr } => {
                self.resolve_expression_id(*expr, ast)?;
            }
            Stmt::Return {
                keyword: _keyword,
                value,
            } => {
                if self.current_function == FunctionType::None {
                    bail!("Can't return from top-level code.");
                }

                if let Some(expr) = value {
                    self.resolve_expression_id(*expr, ast)?;
                }
            }
            Stmt::Var {
                name,
                initializer,
                env_location,
            } => {
                self.declare(name)?;
                if let Some(initializer_expression) = initializer {
                    self.resolve_expression_id(*initializer_expression, ast)?;
                }

                // If this declaration is local, record its slot.
                if let Some(scope) = self.scopes.last()
                    && let Some((slot, _)) = scope.get(name)
                {
                    env_location.set(Some((0, *slot)));
                }

                self.define(name);
            }
            Stmt::Block { statements } => {
                self.begin_scope();
                for statement in statements {
                    self.resolve_statement_id(*statement, ast)?;
                }
                self.end_scope();
            }
            Stmt::While { condition, body } => {
                self.resolve_expression_id(*condition, ast)?;
                self.resolve_statement_id(*body, ast)?;
            }
            Stmt::Function {
                name,
                params,
                body,
                env_location,
            } => {
                self.declare(&name.lexeme)?;
                if let Some(scope) = self.scopes.last()
                    && let Some((slot, _is_defined)) = scope.get(&name.lexeme)
                {
                    env_location.set(Some((0, *slot)));
                }
                self.define(&name.lexeme);
                self.resolve_function(params, body, ast)?;
            }
        }

        Ok(())
    }

    fn resolve_expression(&mut self, expr: &Expr, ast: &Ast) -> Result<()> {
        match expr {
            Expr::Logical {
                left,
                op: _op,
                right,
            } => {
                self.resolve_expression_id(*left, ast)?;
                self.resolve_expression_id(*right, ast)?;
            }
            Expr::Binary {
                left,
                op: _op,
                right,
            } => {
                self.resolve_expression_id(*left, ast)?;
                self.resolve_expression_id(*right, ast)?;
            }
            Expr::Call {
                callee,
                paren: _paren,
                arguments,
            } => {
                self.resolve_expression_id(*callee, ast)?;
                for arg in arguments {
                    self.resolve_expression_id(*arg, ast)?;
                }
            }
            Expr::Assign {
                name,
                value,
                env_location,
            } => {
                self.resolve_expression_id(*value, ast)?;
                self.resolve_local(name, env_location);
            }
            Expr::Literal { value: _ } => (),
            Expr::Unary { op: _op, right } => {
                self.resolve_expression_id(*right, ast)?;
            }
            Expr::Variable { name, env_location } => {
                if let Some(scope) = self.scopes.last()
                    && scope.get(name).is_some_and(|(_, is_defined)| !*is_defined)
                {
                    bail!("Can't read local variable in its own initializer.");
                }
                self.resolve_local(name, env_location);
            }
            Expr::Grouping { value } => self.resolve_expression_id(*value, ast)?,
        }

        Ok(())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn declare(&mut self, name: &str) -> Result<()> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(name) {
                bail!("Already a variable with this name in this scope.");
            }

            let slot = scope.len();
            scope.insert(name.to_owned(), (slot, false));
        }

        Ok(())
    }

    fn define(&mut self, name: &String) {
        if let Some(scope) = self.scopes.last_mut()
            && let Some((_slot, value)) = scope.get_mut(name)
        {
            *value = true;
        }
    }

    pub fn resolve(&mut self, ast: &Ast) -> Result<()> {
        for statement in &ast.statements {
            self.resolve_statement(statement, ast)?;
        }

        Ok(())
    }

    // NOTE: in they book the keep this in a seperate map in the interpreter.
    // Reason, otherwise rewrite was needed -- extra pages and ink.
    // Limitation does not exist here, so I store in AST node itself.
    #[allow(clippy::arithmetic_side_effects)]
    fn resolve_local(&self, name: &str, env_location: &Cell<Option<(Depth, Slot)>>) {
        for (index, scope) in self.scopes.iter().enumerate().rev() {
            if let Some((slot, _)) = scope.get(name) {
                let depth = self.scopes.len() - 1 - index;
                // Retrieve the slot form the scope that was defined during declaration

                env_location.set(Some((depth, *slot)));
                return;
            }
        }
    }

    // TODO lookup other places where I use &Vec<> as param instead of &[] __ slice
    fn resolve_function(
        &mut self,
        params: &[super::token::Token],
        body: &[StmtId],
        ast: &Ast,
    ) -> Result<()> {
        let enclosing_function = self.current_function;
        self.current_function = FunctionType::Function;
        self.begin_scope();

        /*  https://stackoverflow.com/questions/2321511/what-is-meant-by-resource-acquisition-is-initialization-raii
        NOTE: We wrapt the for loops (resolving function_body) in a result producing closure.
        the error returns early _ ? returns early _ but is capture in the closure
        Otherwise we return early and don't execute the rest of the function

        self.end_scope()
        self.current_function = enclosing function

        leaving the resolver in a poisened state.
        I don't have error recovery, so early return doesn't really matter...
        But still a good pattern to know about...
         */
        let result = (|| {
            for param in params {
                self.declare(&param.lexeme)?;
                self.define(&param.lexeme);
            }

            for statement in body {
                self.resolve_statement_id(*statement, ast)?;
            }
            Ok(())
        })();

        self.end_scope();
        self.current_function = enclosing_function;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::Literal;
    use crate::frontend::token::{Token, TokenType};
    use std::cell::Cell;

    #[test]
    fn resolve_sets_variable_scope_depth() {
        let statements = vec![Stmt::Block {
            statements: vec![
                Stmt::Var {
                    name: "a".to_string(),
                    initializer: Some(Expr::Literal {
                        value: Literal::Float(1.0),
                    }),
                    env_location: Cell::new(None),
                },
                Stmt::Block {
                    statements: vec![Stmt::Var {
                        name: "b".to_string(),
                        initializer: Some(Expr::Variable {
                            name: "a".to_string(),
                            env_location: Cell::new(None),
                        }),
                        env_location: Cell::new(None),
                    }],
                },
            ],
        }];

        let mut resolver = Resolver::default();
        resolver.resolve(&statements).unwrap();

        let inner_block = match &statements[0] {
            Stmt::Block { statements } => statements,
            _ => unreachable!(),
        };

        let inner_var = match &inner_block[1] {
            Stmt::Block { statements } => match &statements[0] {
                Stmt::Var {
                    initializer: Some(Expr::Variable { env_location, .. }),
                    ..
                } => env_location,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        assert_eq!(inner_var.get(), Some((1, 0)));
    }

    #[test]
    fn resolve_sets_same_scope_depth_and_slot() {
        let statements = vec![Stmt::Block {
            statements: vec![
                Stmt::Var {
                    name: "a".to_string(),
                    initializer: None,
                    env_location: Cell::new(None),
                },
                Stmt::Var {
                    name: "b".to_string(),
                    initializer: Some(Expr::Variable {
                        name: "a".to_string(),
                        env_location: Cell::new(None),
                    }),
                    env_location: Cell::new(None),
                },
                Stmt::ExpressionStmt {
                    expr: Expr::Variable {
                        name: "b".to_string(),
                        env_location: Cell::new(None),
                    },
                },
            ],
        }];

        let mut resolver = Resolver::default();
        resolver.resolve(&statements).unwrap();

        let block_statements = match &statements[0] {
            Stmt::Block { statements } => statements,
            _ => unreachable!(),
        };
        let a_reference = match &block_statements[1] {
            Stmt::Var {
                initializer: Some(Expr::Variable { env_location, .. }),
                ..
            } => env_location,
            _ => unreachable!(),
        };
        let b_reference = match &block_statements[2] {
            Stmt::ExpressionStmt {
                expr: Expr::Variable { env_location, .. },
            } => env_location,
            _ => unreachable!(),
        };

        assert_eq!(a_reference.get(), Some((0, 0)));
        assert_eq!(b_reference.get(), Some((0, 1)));
    }

    #[test]
    fn resolve_sets_depth_for_multiple_enclosing_scopes() {
        let statements = vec![Stmt::Block {
            statements: vec![
                Stmt::Var {
                    name: "a".to_string(),
                    initializer: None,
                    env_location: Cell::new(None),
                },
                Stmt::Block {
                    statements: vec![Stmt::Block {
                        statements: vec![Stmt::ExpressionStmt {
                            expr: Expr::Variable {
                                name: "a".to_string(),
                                env_location: Cell::new(None),
                            },
                        }],
                    }],
                },
            ],
        }];

        let mut resolver = Resolver::default();
        resolver.resolve(&statements).unwrap();

        let reference = match &statements[0] {
            Stmt::Block { statements } => match &statements[1] {
                Stmt::Block { statements } => match &statements[0] {
                    Stmt::Block { statements } => match &statements[0] {
                        Stmt::ExpressionStmt {
                            expr: Expr::Variable { env_location, .. },
                        } => env_location,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        assert_eq!(reference.get(), Some((2, 0)));
    }

    #[test]
    fn resolve_leaves_global_variable_unresolved() {
        let statements = vec![Stmt::ExpressionStmt {
            expr: Expr::Variable {
                name: "global".to_string(),
                env_location: Cell::new(None),
            },
        }];

        let mut resolver = Resolver::default();
        resolver.resolve(&statements).unwrap();

        let reference = match &statements[0] {
            Stmt::ExpressionStmt {
                expr: Expr::Variable { env_location, .. },
            } => env_location,
            _ => unreachable!(),
        };

        assert_eq!(reference.get(), None);
    }

    #[test]
    fn resolve_sets_assignment_scope_depth_and_slot() {
        let statements = vec![Stmt::Block {
            statements: vec![
                Stmt::Var {
                    name: "a".to_string(),
                    initializer: None,
                    env_location: Cell::new(None),
                },
                Stmt::Block {
                    statements: vec![Stmt::ExpressionStmt {
                        expr: Expr::Assign {
                            name: "a".to_string(),
                            value: Box::new(Expr::Literal {
                                value: Literal::Float(2.0),
                            }),
                            env_location: Cell::new(None),
                        },
                    }],
                },
            ],
        }];

        let mut resolver = Resolver::default();
        resolver.resolve(&statements).unwrap();

        let assignment = match &statements[0] {
            Stmt::Block { statements } => match &statements[1] {
                Stmt::Block { statements } => match &statements[0] {
                    Stmt::ExpressionStmt {
                        expr: Expr::Assign { env_location, .. },
                    } => env_location,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        assert_eq!(assignment.get(), Some((1, 0)));
    }

    #[test]
    fn resolve_uses_nearest_shadowing_declaration() {
        let statements = vec![Stmt::Block {
            statements: vec![
                Stmt::Var {
                    name: "a".to_string(),
                    initializer: None,
                    env_location: Cell::new(None),
                },
                Stmt::Block {
                    statements: vec![
                        Stmt::Var {
                            name: "a".to_string(),
                            initializer: None,
                            env_location: Cell::new(None),
                        },
                        Stmt::ExpressionStmt {
                            expr: Expr::Variable {
                                name: "a".to_string(),
                                env_location: Cell::new(None),
                            },
                        },
                    ],
                },
            ],
        }];

        let mut resolver = Resolver::default();
        resolver.resolve(&statements).unwrap();

        let reference = match &statements[0] {
            Stmt::Block { statements } => match &statements[1] {
                Stmt::Block { statements } => match &statements[1] {
                    Stmt::ExpressionStmt {
                        expr: Expr::Variable { env_location, .. },
                    } => env_location,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        assert_eq!(reference.get(), Some((0, 0)));
    }

    #[test]
    fn resolve_sets_scope_depth_for_call_callee_variable() {
        let statements = vec![Stmt::Block {
            statements: vec![
                Stmt::Function {
                    name: Token::new(TokenType::Identifier, "show".to_string(), 1),
                    params: vec![],
                    body: vec![],
                    env_location: Cell::new(None),
                },
                Stmt::ExpressionStmt {
                    expr: Expr::Call {
                        callee: Box::new(Expr::Variable {
                            name: "show".to_string(),
                            env_location: Cell::new(None),
                        }),
                        paren: TokenType::RightParen,
                        arguments: vec![],
                    },
                },
            ],
        }];

        let mut resolver = Resolver::default();
        resolver.resolve(&statements).unwrap();

        let block_statements = match &statements[0] {
            Stmt::Block { statements } => statements,
            _ => unreachable!(),
        };

        let callee_scope_depth = match &block_statements[1] {
            Stmt::ExpressionStmt {
                expr:
                    Expr::Call {
                        callee,
                        arguments: _,
                        paren: _,
                    },
            } => match callee.as_ref() {
                Expr::Variable { env_location, .. } => env_location,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        assert_eq!(callee_scope_depth.get(), Some((0, 0)));
    }

    #[test]
    fn resolve_propagates_errors_from_function_body() {
        let statements = vec![Stmt::Function {
            name: Token::new(TokenType::Identifier, "show".to_string(), 1),
            params: vec![],
            body: vec![Stmt::Var {
                name: "a".to_string(),
                initializer: Some(Expr::Variable {
                    name: "a".to_string(),
                    env_location: Cell::new(None),
                }),
                env_location: Cell::new(None),
            }],
            env_location: Cell::new(None),
        }];

        let mut resolver = Resolver::default();
        let error = resolver.resolve(&statements).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Can't read local variable in its own initializer."
        );
    }

    #[test]
    fn resolve_rejects_duplicate_local_declarations() {
        let statements = vec![Stmt::Block {
            statements: vec![
                Stmt::Var {
                    name: "a".to_string(),
                    initializer: None,
                    env_location: Cell::new(None),
                },
                Stmt::Var {
                    name: "a".to_string(),
                    initializer: None,
                    env_location: Cell::new(None),
                },
            ],
        }];

        let mut resolver = Resolver::default();
        let error = resolver.resolve(&statements).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Already a variable with this name in this scope."
        );
    }

    #[test]
    fn resolve_rejects_top_level_return() {
        let statements = vec![Stmt::Return {
            keyword: Token::new(TokenType::Return, "return".to_string(), 1),
            value: Some(Expr::Literal {
                value: Literal::Str("value".to_string()),
            }),
        }];

        let mut resolver = Resolver::default();
        let error = resolver.resolve(&statements).unwrap_err();

        assert_eq!(error.to_string(), "Can't return from top-level code.");
    }

    #[test]
    fn resolve_allows_return_inside_function() {
        let statements = vec![Stmt::Function {
            name: Token::new(TokenType::Identifier, "show".to_string(), 1),
            params: vec![],
            body: vec![Stmt::Return {
                keyword: Token::new(TokenType::Return, "return".to_string(), 1),
                value: Some(Expr::Literal {
                    value: Literal::Float(1.0),
                }),
            }],
            env_location: Cell::new(None),
        }];

        let mut resolver = Resolver::default();
        resolver.resolve(&statements).unwrap();
    }
}
