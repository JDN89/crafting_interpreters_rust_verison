use anyhow::Context;
use anyhow::Result;

use crate::frontend::ast::{Expr, Operator};
use crate::frontend::token::{Token, TokenType};

// TODO: again. get rid of the lifetime. We are draggin &str subslices around beter to something like struct Span { start: u32, len: u32 } and then slice in to the source code if I actually need the source code which I don't think i do at the moment... Rip it out?
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
        return self.peek().ttype == ttype;
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous();
    }

    fn peek(&self) -> &Token {
        self.tokens
            .iter()
            .nth(self.current)
            .expect("Error indexing into tokens")
    }

    fn is_at_end(&self) -> bool {
        return self.peek().ttype == TokenType::Eof;
    }

    fn previous(&self) -> &Token {
        return self
            .tokens
            .iter()
            .nth(self.current - 1)
            .expect("Error indexing into Parser::tokens");
    }

    fn comparison(&self) -> Expr {
        todo!()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison();

        while self.match_ttype(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let token = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: Operator::from_token_type(token.ttype)
                    .context("Could not convert token type to operator")?,
                right: Box::new(right),
            }
        }
        return Ok(expr);
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }
}
