use anyhow::{Result, bail};

use phf::phf_map;

use crate::frontend::token::{Token, TokenType};

// NOTE [source code phf](https://docs.rs/phf/latest/phf/)

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

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

    fn add_token(&mut self, ttype: TokenType) {
        // NOTE: range exclusive omdat current al advanced is naar de volgende positie, door
        // self.advance()
        let text = &self.source[self.start..self.current];
        let token = Token::new(ttype, text.to_string(), self.line);
        self.tokens.push(token);
    }

    fn match_char(&mut self, expected: char) -> bool {
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

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current + 1)
            .expect("Error in Lexer::peek_next()")
    }

    fn string(&mut self) -> Result<()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            bail!("[line {}] Error : Unterminated string", self.line);
        }

        // NOTE consume the enclosing "
        self.advance();

        self.add_token(TokenType::String);

        Ok(())
    }

    pub fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    // NOTE The litereal String I borrow, beucase it reflects the source code value. With number we convert the source code to a number so here it doesn't make sense to borrow.
    pub fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        self.add_token(TokenType::Number);
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
    }

    fn is_alpha_numberic(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }

    fn identifier(&mut self) {
        while self.is_alpha_numberic(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let ttype = KEYWORDS.get(text);
        match ttype {
            Some(value) => self.add_token(*value),
            None => self.add_token(TokenType::Identifier),
        }
    }

    fn scan_token(&mut self) -> Result<()> {
        // NOTE This call to advance also consumes the default error line
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_matches_equal = self.match_char('=');
                if token_matches_equal {
                    self.add_token(TokenType::BangEqual);
                } else {
                    // NOTE when it doesn't match current doesn't advance
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                let token_matches_equal = self.match_char('=');
                if token_matches_equal {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                let token_matches_equal = self.match_char('=');
                if token_matches_equal {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                let token_matches_equal = self.match_char('=');
                if token_matches_equal {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                // NOTE '//' is a comment and goes till the end of the line
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                // NOTE /* is block comment */
                else if self.match_char('*') {
                    // NOTE consume al chars until we encounter the closing '*/'
                    while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
                        if self.peek() == '\n' {
                            self.line += 1;
                        }
                        self.advance();
                    }

                    // NOTE consume '*/' characters
                    self.advance();
                    self.advance();

                    // NOTE bug if we are at end or if we don't find the terminating '/' for the block
                    // comment
                    if self.is_at_end() {
                        bail!("[Line {}] Error: Unterminated block comment!", self.line);
                    }
                } else {
                    // NOTE only '/' for division
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string()?,
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    bail!("[line {}] Error : Unexpected character", self.line);
                }
            }
        }
        Ok(())
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), self.line));

        Ok(self.tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        let lexer = Lexer::new("");
        let tokens = lexer.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].ttype, TokenType::Eof);
    }

    #[test]
    fn test_single_left_paren() {
        let lexer = Lexer::new("(");
        let tokens = lexer.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 2); // LeftParen + Eof
        assert_eq!(tokens[0].ttype, TokenType::LeftParen);
        assert_eq!(tokens[0].lexeme, "(");
        assert_eq!(tokens[1].ttype, TokenType::Eof);
    }

    #[test]
    fn test_multiple_left_parens() {
        let lexer = Lexer::new("(((");
        let tokens = lexer.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 4); // 3 LeftParens + Eof
        for i in 0..3 {
            assert_eq!(tokens[i].ttype, TokenType::LeftParen);
        }
        assert_eq!(tokens[3].ttype, TokenType::Eof);
    }

    #[test]
    fn test_unexpected_character_error() {
        let lexer = Lexer::new("$");
        let result = lexer.scan_tokens();

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Unexpected character"));
    }

    #[test]
    fn test_line_tracking() {
        let lexer = Lexer::new("(");
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
            let lexer = Lexer::new(input);
            let tokens = lexer.scan_tokens().unwrap();
            assert_eq!(tokens[0].ttype, expected_type);
        }
    }

    #[test]
    fn test_comment() {
        let input = "// This is a comment";
        let lexer = Lexer::new(input);
        let tokens = lexer.scan_tokens().unwrap();
        assert!(tokens.len() == 1);
        assert_eq!(tokens[0].ttype, TokenType::Eof);
    }

    #[test]
    fn test_block_comment() {
        let input = "/* This is a block comment

            and has a newline
            */
            ";
        let lexer = Lexer::new(input);
        let tokens = lexer.scan_tokens().unwrap();
        assert!(tokens.len() == 1);
        assert_eq!(tokens[0].ttype, TokenType::Eof);
    }

    //TODO merge with above and make one consecutive test
    // #[test]
    // fn test_block_comment_part2() {
    //     let input = "/* This is a block comment
    //
    //         and has a newline
    //         *|
    //         ";
    //     let mut lexer = Lexer::new(input);
    //     let tokens = lexer.scan_tokens().unwrap();
    //     assert!(tokens.len() == 1);
    //     assert_eq!(tokens[0].ttype, TokenType::Eof);
    // }

    #[test]
    fn test_whitespaces_and_return() {
        let input = "    

          (";
        let lexer = Lexer::new(input);
        let tokens = lexer.scan_tokens().unwrap();
        assert!(tokens.len() == 2);
        assert_eq!(tokens[0].ttype, TokenType::LeftParen);
        assert_eq!(tokens[1].ttype, TokenType::Eof);
    }

    #[test]
    fn test_string() {
        let input = "\" yolo\"";

        let lexer = Lexer::new(input);
        let tokens = lexer.scan_tokens().unwrap();
        assert!(tokens.len() == 2);
        assert_eq!(tokens[0].ttype, TokenType::String);
        assert_eq!(tokens[1].ttype, TokenType::Eof);
    }

    #[test]
    fn test_keyword() {
        let test_cases = vec![
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
            ("identifier", TokenType::Identifier),
        ];

        for (input, ttype) in test_cases {
            let lexer = Lexer::new(input);
            let tokens = lexer.scan_tokens().unwrap();
            assert!(tokens.len() == 2);
            assert_eq!(tokens[0].ttype, ttype);
            assert_eq!(tokens[0].lexeme, input);
            assert_eq!(tokens[1].ttype, TokenType::Eof);
        }
    }

    #[test]
    fn test_number() {
        let input = "123.45";
        let lexer = Lexer::new(input);
        let tokens = lexer.scan_tokens().unwrap();
        assert!(tokens.len() == 2);
        assert_eq!(tokens[0].ttype, TokenType::Number);
    }
}
