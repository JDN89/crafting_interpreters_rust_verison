use std::cell::Cell;

use anyhow::{Context, Result, bail};

use crate::frontend::ast::Literal;
use crate::frontend::ast::Stmt;
use crate::frontend::ast::{Expr, Operator};
use crate::frontend::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<String>,
}

impl Parser {
    #[must_use]
    pub const fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    /// Checks whether the current token matches any token type in `types`.
    ///
    /// If a match is found, this consumes the token by advancing the parser
    /// and returns `true`. Otherwise, it leaves the parser unchanged and
    /// returns `false`.
    fn match_ttype(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Checks whether the current token matches any token type in `types`.
    /// and return true or false
    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().ttype == ttype
    }

    /// advance and return consumed Token
    #[allow(clippy::arithmetic_side_effects)]
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous();
    }

    #[allow(clippy::expect_used)]
    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .expect("Error indexing into tokens")
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }

    /// return previous Token
    #[allow(clippy::expect_used, clippy::arithmetic_side_effects)]
    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current - 1)
            .expect("Error indexing into Parser::tokens")
    }

    fn comparison(&mut self) -> Result<Expr, anyhow::Error> {
        let mut expr = self.term()?;
        while self.match_ttype(&[TokenType::Greater, TokenType::Less, TokenType::LessEqual]) {
            // NOTE TokenType is of type copy. If we returned &token we run into borrowing issues
            //Rust tracks borrows based on how long a reference is stored, not how long the function call lasts.
            let operator_type = self.previous().ttype;
            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op: Operator::from_token_type(operator_type)
                    .context("Could not convert token type to operator")?,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, anyhow::Error> {
        let mut expr = self.comparison()?;

        while self.match_ttype(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator_type = self.previous().ttype;
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: Operator::from_token_type(operator_type)
                    .context("Could not convert token type to operator")?,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr, anyhow::Error> {
        self.assignment()
    }

    fn term(&mut self) -> Result<Expr, anyhow::Error> {
        let mut expr = self.factor()?;
        while self.match_ttype(&[TokenType::Minus, TokenType::Plus]) {
            let operator_type = self.previous().ttype;
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: Operator::from_token_type(operator_type)
                    .context("Could not convert token type to operator")?,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, anyhow::Error> {
        let mut expr = self.unary()?;
        while self.match_ttype(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().ttype;
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: Operator::from_token_type(operator)
                    .context("Could not convert token type to operator")?,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, anyhow::Error> {
        if self.match_ttype(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().ttype;
            let right = self.unary()?;
            Ok(Expr::Unary {
                op: Operator::from_token_type(operator)
                    .context("Could not convert token type to operator")?,
                right: Box::new(right),
            })
        } else {
            self.call()
        }
    }

    #[allow(clippy::expect_used, clippy::indexing_slicing)]
    fn primary(&mut self) -> Result<Expr, anyhow::Error> {
        if self.match_ttype(&[TokenType::False]) {
            return Ok(Expr::Literal {
                value: Literal::Boolean(false),
            });
        }
        if self.match_ttype(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: Literal::Boolean(true),
            });
        }
        if self.match_ttype(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: Literal::Nil,
            });
        }

        if self.match_ttype(&[TokenType::String]) {
            return Ok(Expr::Literal {
                value: Literal::Str(self.previous().lexeme.clone()),
            });
        }
        if self.match_ttype(&[TokenType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous().lexeme.clone(),
                env_location: Cell::new(None),
            });
        }

        if self.match_ttype(&[TokenType::Number]) {
            return Ok(Expr::Literal {
                value: Literal::Float(
                    self.previous()
                        .lexeme
                        .parse()
                        .expect("Invalid number literal"),
                ),
            });
        }
        if self.match_ttype(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping {
                value: Box::new(expr),
            });
        }

        bail!(
            "Expected expression. But got token {:?}",
            self.tokens[self.current]
        )
    }

    fn consume(&mut self, ttype: TokenType, arg: &str) -> Result<()> {
        if self.check(ttype) {
            self.advance();
            Ok(())
        } else {
            bail!("{arg}.")
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            if let Some(statement) = self.parse_declaration() {
                statements.push(statement);
            }
        }

        if self.errors.is_empty() {
            Ok(statements)
        } else {
            bail!(self.errors.join("\n"))
        }
    }

    fn parse_statement(&mut self) -> Result<Stmt> {
        if self.match_ttype(&[TokenType::For]) {
            self.parse_for_statement()
        } else if self.match_ttype(&[TokenType::If]) {
            self.consume(TokenType::LeftParen, "Expect '(' after 'if'")?;
            let condition = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;
            let then_branch: Stmt = self.parse_statement()?;
            let else_branch = if self.match_ttype(&[TokenType::Else]) {
                Some(Box::new(self.parse_statement()?))
            } else {
                None
            };

            Ok(Stmt::IfStatement {
                condition,
                then_branch: Box::new(then_branch),
                else_branch,
            })
        } else if self.match_ttype(&[TokenType::Print]) {
            self.parse_print_statement()
        } else if self.match_ttype(&[TokenType::Return]) {
            self.parse_return_statement()
        } else if self.match_ttype(&[TokenType::While]) {
            self.parse_while_statement()
        } else if self.match_ttype(&[TokenType::LeftBrace]) {
            self.parse_block_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    fn parse_declaration(&mut self) -> Option<Stmt> {
        let result = if self.match_ttype(&[TokenType::Fun]) {
            self.parse_function()
        } else if self.match_ttype(&[TokenType::Var]) {
            self.parse_var_declaration()
        } else {
            self.parse_statement()
        };

        match result {
            Ok(statement) => Some(statement),
            Err(err) => {
                self.errors.push(err.to_string());
                self.synchronize();
                None
            }
        }
    }

    fn parse_print_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::PrintStmt { expr })
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::ExpressionStmt { expr })
    }

    fn parse_var_declaration(&mut self) -> Result<Stmt> {
        self.consume(TokenType::Identifier, "Expect variable name")?;
        let name = self.previous().lexeme.clone();

        let initializer = if self.match_ttype(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var {
            name,
            initializer,
            env_location: Cell::new(None),
        })
    }

    /// left side is first parsed as an expression -> should be an `Expr::variable` (Identifier,
    /// where we store the value)
    /// if next token is '=' start parsing assignement expression
    /// parse right hand side wich should also be an expression
    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.or()?;

        if self.match_ttype(&[TokenType::Equal]) {
            let equals = self.previous().clone(); // cloning token is cheap
            let value = self.assignment()?;
            if let Expr::Variable { name, .. } = &expr {
                return Ok(Expr::Assign {
                    name: name.clone(),
                    value: Box::new(value),
                    env_location: Cell::new(None),
                });
            }
            bail!("Token: {equals} is an invalid assingment target.");
        }
        Ok(expr)
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(statement) = self.parse_declaration() {
                statements.push(statement);
            }
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block")?;
        Ok(statements)
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ttype == TokenType::Semicolon {
                return;
            }

            match self.peek().ttype {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => self.advance(),
            }
        }
    }

    fn parse_block_statement(&mut self) -> Result<Stmt> {
        let statements = self.parse_block()?;
        Ok(Stmt::Block { statements })
    }

    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;

        while self.match_ttype(&[TokenType::Or]) {
            let op = TokenType::Or;
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;
        while self.match_ttype(&[TokenType::And]) {
            let op = TokenType::And;
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn parse_while_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.parse_statement()?;
        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn parse_for_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'")?;

        // initializer
        let initializer = if self.match_ttype(&[TokenType::Semicolon]) {
            None
        } else if self.match_ttype(&[TokenType::Var]) {
            Some(self.parse_var_declaration()?)
        } else {
            Some(self.parse_expression_statement()?)
        };

        // condition
        let condition = if self.check(TokenType::Semicolon) {
            self.consume(TokenType::Semicolon, "Expect ';' after loop condition")?;
            None
        } else {
            let expr = Some(self.expression()?);
            self.consume(TokenType::Semicolon, "Expect ';' after loop condition")?;
            expr
        };

        // increment
        let increment = if self.check(TokenType::RightParen) {
            self.consume(TokenType::RightParen, "Expect ')' after for clause")?;
            None
        } else {
            let expr = Some(self.expression()?);
            self.consume(TokenType::RightParen, "Expect ')' after for clause")?;
            expr
        };

        // loop body
        let mut body = self.parse_statement()?;

        // append increment after each iteration
        if let Some(increment) = increment {
            body = Stmt::Block {
                statements: vec![body, Stmt::ExpressionStmt { expr: increment }],
            };
        }

        // if condition is missing we default to true
        let condition = condition.unwrap_or(Expr::Literal {
            value: Literal::Boolean(true),
        });

        // Next, we take the condition and the body and build the loop using a primitive while loop
        body = Stmt::While {
            condition,
            body: Box::new(body),
        };

        // prepend initializer
        if let Some(initializer) = initializer {
            body = Stmt::Block {
                statements: vec![initializer, body],
            };
        }

        Ok(body)
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        while self.match_ttype(&[TokenType::LeftParen]) {
            expr = self.finish_call(expr)?;
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);

                if !self.match_ttype(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren: TokenType::RightParen,
            arguments,
        })
    }

    // 10.3 function declaration
    fn parse_function(&mut self) -> Result<Stmt> {
        self.consume(TokenType::Identifier, "Expect function name")?;
        let name: Token = self.previous().clone();

        self.consume(TokenType::LeftParen, "Expect '(' after function name")?;

        let mut parameters: Vec<Token> = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    bail!("Can't have more than 255 parameters");
                }
                self.consume(TokenType::Identifier, "Expect parameter name.")?;
                let param = self.previous().clone();

                parameters.push(param);

                if !self.match_ttype(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(TokenType::LeftBrace, "Expect '{' before function body.")?;
        let statements = self.parse_block()?;

        Ok(Stmt::Function {
            name,
            params: parameters,
            body: statements,
            env_location: Cell::new(None),
        })
    }

    fn parse_return_statement(&mut self) -> Result<Stmt> {
        let keyword = self.previous().clone();
        let value = if self.check(TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;

        Ok(Stmt::Return { keyword, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::Lexer;
    use crate::frontend::token::Token;

    #[test]
    fn parse_call_expression() {
        let tokens = Lexer::new("foo(1, 2);").scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            Stmt::ExpressionStmt {
                expr: Expr::Call {
                    callee: Box::new(Expr::Variable {
                        name: "foo".to_string(),
                        env_location: Cell::new(None),
                    }),
                    paren: TokenType::RightParen,
                    arguments: vec![
                        Expr::Literal {
                            value: Literal::Float(1.0),
                        },
                        Expr::Literal {
                            value: Literal::Float(2.0),
                        },
                    ],
                },
            }
        );
    }

    #[test]
    fn parse_function_declaration_with_parameters() {
        let tokens = Lexer::new("fun add(a, b) { print a; }")
            .scan_tokens()
            .unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            Stmt::Function {
                name: Token::new(TokenType::Identifier, "add".to_string(), 1),
                params: vec![
                    Token::new(TokenType::Identifier, "a".to_string(), 1),
                    Token::new(TokenType::Identifier, "b".to_string(), 1),
                ],
                body: vec![Stmt::PrintStmt {
                    expr: Expr::Variable {
                        name: "a".to_string(),
                        env_location: Cell::new(None),
                    },
                }],
                env_location: Cell::new(None),
            }
        );
    }

    #[test]
    fn parse_return_statement_with_value() {
        let tokens = Lexer::new("return 123;").scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            Stmt::Return {
                keyword: Token::new(TokenType::Return, "return".to_string(), 1),
                value: Some(Expr::Literal {
                    value: Literal::Float(123.0),
                }),
            }
        );
    }

    #[test]
    fn parse_return_statement_without_value() {
        let tokens = Lexer::new("return;").scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            Stmt::Return {
                keyword: Token::new(TokenType::Return, "return".to_string(), 1),
                value: None,
            }
        );
    }

    #[test]
    fn recovers_after_syntax_error_and_parses_next_statement() {
        let tokens = Lexer::new("var a = ; print 123;").scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let mut statements = Vec::new();

        while !parser.is_at_end() {
            if let Some(statement) = parser.parse_declaration() {
                statements.push(statement);
            }
        }

        assert_eq!(parser.errors.len(), 1);
        assert_eq!(
            statements,
            vec![Stmt::PrintStmt {
                expr: Expr::Literal {
                    value: Literal::Float(123.0),
                },
            }]
        );
    }
}
