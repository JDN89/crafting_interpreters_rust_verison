use anyhow::*;

use crate::frontend::token::{Token, TokenType};

pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self
            .source
            .chars()
            .nth(self.current)
            .expect("Error advancing the lexer and indexing into the source code");
        self.current += 1;
        c
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<String>) {
        // NOTE: range exclusive omdat current al advanced is naar de volgende positie, door
        // self.advance()
        let text = &self.source[self.start..self.current];
        let token = Token::new(ttype, text.to_string(), literal, self.line);
        self.tokens.push(token);
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self
            .source
            .chars()
            .nth(self.current)
            .expect("Error at match_token whilst indexing into self.source")
            != expected
        {
            return false;
        }
        // NOTE We only conly consume the current character when it matches with the expected token
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current)
            .expect("Error in lexer.peek()")
    }

    fn scan_token(&mut self) -> Result<()> {
        // NOTE This call to advance also consumes the default error line
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let token_matches_equal = self.match_token('=');
                if token_matches_equal {
                    self.add_token(TokenType::BangEqual, None);
                } else {
                    // NOTE when it doesn't match current doesn't advance
                    self.add_token(TokenType::Bang, None);
                }
            }
            '=' => {
                let token_matches_equal = self.match_token('=');
                if token_matches_equal {
                    self.add_token(TokenType::EqualEqual, None);
                } else {
                    self.add_token(TokenType::Equal, None);
                }
            }
            '<' => {
                let token_matches_equal = self.match_token('=');
                if token_matches_equal {
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }
            }
            '>' => {
                let token_matches_equal = self.match_token('=');
                if token_matches_equal {
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            }
            '/' => {
                let token_matches_slash = self.match_token('/');
                if token_matches_slash {
                    // NOTE a comment goes until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => {
                return Err(anyhow!("[line {}] Error : Unexpected character", self.line));
            }
        }
        Ok(())
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?
        }
        // TODO: check what is faster "".to_string() or Vec::new()
        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));

        Ok(&self.tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].ttype, TokenType::Eof);
    }

    #[test]
    fn test_single_left_paren() {
        let mut lexer = Lexer::new("(");
        let tokens = lexer.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 2); // LeftParen + Eof
        assert_eq!(tokens[0].ttype, TokenType::LeftParen);
        assert_eq!(tokens[0].lexeme, "(");
        assert_eq!(tokens[1].ttype, TokenType::Eof);
    }

    #[test]
    fn test_multiple_left_parens() {
        let mut lexer = Lexer::new("(((");
        let tokens = lexer.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 4); // 3 LeftParens + Eof
        for i in 0..3 {
            assert_eq!(tokens[i].ttype, TokenType::LeftParen);
        }
        assert_eq!(tokens[3].ttype, TokenType::Eof);
    }

    #[test]
    fn test_unexpected_character_error() {
        let mut lexer = Lexer::new("x");
        let result = lexer.scan_tokens();

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Unexpected character"));
    }

    #[test]
    fn test_line_tracking() {
        let mut lexer = Lexer::new("(");
        let tokens = lexer.scan_tokens().unwrap();

        assert_eq!(tokens[0].line, 1);
    }

    #[test]
    fn test_all_single_char_tokens() {
        let test_cases = vec![
            ("(", TokenType::LeftParen),
            (")", TokenType::RightParen),
            ("{", TokenType::LeftBrace),
            ("}", TokenType::RightBrace),
            (",", TokenType::Comma),
            (".", TokenType::Dot),
            ("-", TokenType::Minus),
            ("+", TokenType::Plus),
            (";", TokenType::Semicolon),
            ("*", TokenType::Star),
            ("!=", TokenType::BangEqual),
            ("!", TokenType::Bang),
        ];

        for (input, expected_type) in test_cases {
            let mut lexer = Lexer::new(input);
            let tokens = lexer.scan_tokens().unwrap();
            assert_eq!(tokens[0].ttype, expected_type);
        }
    }

    #[test]
    fn test_comment() {
        let input = "// This is a comment";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.scan_tokens().unwrap();
        assert!(tokens.len() == 1);
        assert_eq!(tokens[0].ttype, TokenType::Eof);
    }
}
