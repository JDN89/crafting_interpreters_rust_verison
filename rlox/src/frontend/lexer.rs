use crate::frontend::token::Token;

pub struct Lexer<'a> {
    source: &'a str,
    token: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            token: Vec::new(),
        }
    }
    pub fn scan(&self) {
        println!("scanning!");
    }
}
