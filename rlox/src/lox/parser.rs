use crate::LoxError;
use crate::expr::{BinaryExpr, LiteralExpr, UnaryExpr, GroupingExpr};
use crate::token::Literal;
use crate::token_type::TokenType::{self, *};
use crate::{expr::Expr, token::Token};

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code, unused_variables)]
impl Parser {
    fn build_parser(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    // expression     → equality ;
    // todo() : convert to Result<Expr, LoxError> and fix along the chain
    fn expression(&mut self) -> Result<Expr, LoxError> {
        Ok(self.equality()?)
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.match_token_types(&[BangEqual, EqualEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr,LoxError> {
        let mut expr = self.term()?;

        while self.match_token_types(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    // term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<  Expr,LoxError> {
        let mut expr = self.factor()?;
        while self.match_token_types(&[Minus, Plus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.factor()?;

            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    // factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result< Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.match_token_types(&[Slash, Star]) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary()?;

            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    // unary          → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result< Expr, LoxError> {
        if self.match_token_types(&[Bang, Minus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        return Ok(self.primary()?);
    }

    // primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr,LoxError> {
        if self.match_token_types(&[False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Literal::False,
            }));
        }
        if self.match_token_types(&[True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Literal::True,
            }));
        }
        if self.match_token_types(&[Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Literal::Nil,
            }));
        }
        if self.match_token_types(&[Number, String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().unwrap().literal.clone().unwrap(),
            }));

        }

        if self.match_token_types(&[LeftParen]) {
                let expr = self.expression()?;
                self.consume(RightParen, "expect ')' after expression")?;
                return Ok( Expr::Grouping(GroupingExpr{expression: Box::new(expr)}));
            } 
        else {
            todo!()
        }
    }

    // returning the just consumed token makes it easier to use match_token_types
    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn match_token_types(&mut self, token_types: &[crate::token_type::TokenType]) -> bool {
        let mut found_match = false;

        for ttype in token_types {
            if self.check(&ttype) {
                self.advance();
                found_match = true;
            }
        }
        found_match
    }

    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().unwrap().token_type == *ttype
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous().unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.peek().unwrap().token_type == Eof
    }

    fn consume(&mut self, right_paren: TokenType, arg: &str) -> Result<&Token, LoxError> {
        if self.check(&right_paren) {
         return Ok(self.advance())  ;
        }

        let curr_token = self.tokens.get(self.current).unwrap();
      Err(  LoxError::new(curr_token.line,self.current , arg))

    }
}
