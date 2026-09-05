use std::{cell::Cell, collections::HashMap};

use crate::frontend::ast::{Depth, Expr, Slot, Stmt};
use anyhow::{Result, bail};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FunctionType {
    None,
    Function,
}

pub struct Resolver {
    //TODO: store Slot and bool in the hashmap. We store the slot index during the declaration.
    // I was going to determine slot from hashmap len(),which is wrong because by then we might have more elements in the hashmap.
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
    fn resolve_statement(&mut self, statement: &Stmt) -> Result<()> {
        match statement {
            Stmt::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expression(condition)?;
                self.resolve_statement(then_branch)?;
                if let Some(else_stmt) = else_branch {
                    self.resolve_statement(else_stmt)?;
                }
            }
            Stmt::ExpressionStmt { expr } | Stmt::PrintStmt { expr } => {
                self.resolve_expression(expr)?;
            }
            Stmt::Return {
                keyword: _keyword,
                value,
            } => {
                if self.current_function == FunctionType::None {
                    bail!("Can't return from top-level code.");
                }

                if let Some(expr) = value {
                    self.resolve_expression(expr)?;
                }
            }
            Stmt::Var { name, initializer } => {
                self.declare(name)?;
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
                self.end_scope();
            }
            Stmt::While { condition, body } => {
                self.resolve_expression(condition)?;
                self.resolve_statement(body)?;
            }
            // TODO: probably name can be String or stirng interned instead of passing the whole token?
            Stmt::Function { name, params, body } => {
                self.declare(&name.lexeme)?;
                self.define(&name.lexeme);
                self.resolve_function(params, body)?;
            }
        }

        Ok(())
    }

    fn resolve_expression(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Logical {
                left,
                op: _op,
                right,
            } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expr::Binary {
                left,
                op: _op,
                right,
            } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expr::Call {
                callee,
                paren: _paren,
                arguments,
            } => {
                self.resolve_expression(callee)?;
                for arg in arguments {
                    self.resolve_expression(arg)?;
                }
            }
            Expr::Assign {
                name,
                value,
                env_location,
            } => {
                self.resolve_expression(value)?;
                self.resolve_local(name, env_location);
            }
            // TODO : I think we will have to add the slot and VAlue to Expr::Literal and fill it in here during the resolving of Expr::Literal
            // probably we have to do the Same for Expr::Assign and Expr::Variable And LoxFunction? FunctionCall
            Expr::Literal { value: _ } => (),
            Expr::Unary { op: _op, right } => {
                self.resolve_expression(right)?;
            }
            //TODO this looks cursed. I forgot what I am doing hre
            Expr::Variable { name, env_location } => {
                if let Some(scope) = self.scopes.last()
                    && scope.get(name).is_some_and(|(_, is_defined)| !*is_defined)
                {
                    bail!("Can't read local variable in its own initializer.");
                }
                self.resolve_local(name, env_location);
            }
            Expr::Grouping { value } => self.resolve_expression(value)?,
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

    pub fn resolve(&mut self, statements: &[Stmt]) -> Result<()> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }

        Ok(())
    }

    // TODO SEE IT's ehre that we resolve the local whcih means set the depth of the var name!!!
    // NOTE: in they book the keep this in a seperate map in the interpreter. Reason, otherwise rewrite was needed -- extra pages and ink. Limitation does not exist here, so I store in AST node itself.
    #[allow(clippy::arithmetic_side_effects)]
    fn resolve_local(&self, name: &str, env_location: &Cell<Option<(Depth, Slot)>>) {
        for (index, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(name) {
                let depth = self.scopes.len() - 1 - index;

                env_location.set(env_location.get().map(|(_, slot)| (depth, slot)));
                return;
            }
        }
    }

    fn resolve_function(&mut self, params: &[super::token::Token], body: &[Stmt]) -> Result<()> {
        let enclosing_function = self.current_function;
        self.current_function = FunctionType::Function;
        self.begin_scope();

        let result = (|| {
            for param in params {
                self.declare(&param.lexeme)?;
                self.define(&param.lexeme);
            }

            self.resolve(body)
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
                },
                Stmt::Block {
                    statements: vec![Stmt::Var {
                        name: "b".to_string(),
                        initializer: Some(Expr::Variable {
                            name: "a".to_string(),
                            env_location: Cell::new(None),
                        }),
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
    fn resolve_sets_scope_depth_for_call_callee_variable() {
        let statements = vec![Stmt::Block {
            statements: vec![
                Stmt::Function {
                    name: Token::new(TokenType::Identifier, "show".to_string(), 1),
                    params: vec![],
                    body: vec![],
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

        assert_eq!(callee_scope_depth.get(), Some((0, 1)));
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
            }],
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
                },
                Stmt::Var {
                    name: "a".to_string(),
                    initializer: None,
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
        }];

        let mut resolver = Resolver::default();
        resolver.resolve(&statements).unwrap();
    }
}
