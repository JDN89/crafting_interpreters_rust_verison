use std::vec;

use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use anyhow::anyhow;

use crate::frontend::ast::Literal;
use crate::frontend::ast::Stmt;
use crate::frontend::ast::{Expr, Operator};
use crate::frontend::token::{Token, TokenType};

// TODO: I just realised that Everytime I allocate a token to an arena i can just pas Vec<i32>
// around faster then passing Vec<&Token> references around. I think ref takes more memory and is a
// bit slower then indexing into an vec?
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn match_ttype(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().ttype == ttype
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous();
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .expect("Error indexing into tokens")
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }

    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current - 1)
            .expect("Error indexing into Parser::tokens")
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        while self.match_ttype(vec![
            TokenType::Greater,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
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

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.match_ttype(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
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

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while self.match_ttype(vec![TokenType::Minus, TokenType::Plus]) {
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

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        while self.match_ttype(vec![TokenType::Slash, TokenType::Star]) {
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

    fn unary(&mut self) -> Result<Expr> {
        if self.match_ttype(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().ttype;
            let right = self.unary()?;
            Ok(Expr::Unary {
                op: Operator::from_token_type(operator)
                    .context("Could not convert token type to operator")?,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.match_ttype(vec![TokenType::False]) {
            return Ok(Expr::Literal {
                value: Literal::Boolean(false),
            });
        }
        if self.match_ttype(vec![TokenType::True]) {
            return Ok(Expr::Literal {
                value: Literal::Boolean(true),
            });
        }
        if self.match_ttype(vec![TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: Literal::Nil,
            });
        }
        if self.match_ttype(vec![TokenType::True]) {
            return Ok(Expr::Literal {
                value: Literal::Boolean(true),
            });
        }

        if self.match_ttype(vec![TokenType::String]) {
            return Ok(Expr::Literal {
                value: Literal::Str(self.previous().lexeme.clone()),
            });
        }
        if self.match_ttype(vec![TokenType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous().lexeme.clone(),
            });
        }

        if self.match_ttype(vec![TokenType::Number]) {
            return Ok(Expr::Literal {
                value: Literal::Float(
                    self.previous()
                        .lexeme
                        .parse()
                        .expect("Invalid number literal"),
                ),
            });
        }
        if self.match_ttype(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping {
                value: Box::new(expr),
            });
        }

        Err(anyhow!(
            "Expected expression. But got token {:?}",
            self.tokens[self.current]
        ))
    }

    // TODO implement panic mode
    fn consume(&mut self, ttype: TokenType, arg: &str) -> Result<()> {
        if self.check(ttype) {
            self.advance();
            Ok(())
        } else {
            Err(anyhow!("{}.", arg))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_declaration()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt> {
        if self.match_ttype(vec![TokenType::Print]) {
            self.parse_print_statement()
        } else if self.match_ttype(vec![TokenType::LeftBrace]) {
            self.parse_block_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    // TODO: voorzie error recovery -> consume all tokens until next statement declaration
    fn parse_declaration(&mut self) -> Result<Stmt> {
        if self.match_ttype(vec![TokenType::Var]) {
            Ok(self.parse_var_declaration()?)
        } else {
            self.parse_statement()
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

        let initializer = if self.match_ttype(vec![TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    /// left side is first parsed as an expression -> should be an Expr::variable (Identifier,
    /// where we store the value)
    /// if next token is '=' start parsing assignement expression
    /// parse right hand side wich should also be an expression
    fn assignment(&mut self) -> Result<Expr> {
        let expression = self.equality()?;

        if self.match_ttype(vec![TokenType::Equal]) {
            let equals = self.previous().clone(); // cloning token is cheap
            let value = self.assignment()?;
            if let Expr::Variable { name } = &expression {
                return Ok(Expr::Assign {
                    name: name.clone(),
                    value: Box::new(value),
                });
            }
            anyhow::bail!("Token: {} is an invalid assingment target.", &equals);
        }
        Ok(expression)
    }

    // BUG: For some reason when we pass at the moment the TokenType RightBrace to
    // parse_declaration. so the token doesn't
    fn parse_block_statement(&mut self) -> Result<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_declaration()?)
        }
        Ok(Stmt::Block { statements })
    }
}
