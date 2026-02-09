use core::fmt;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Literal {
    Str(String),
    Boolean(bool),
    Float(f64),
}

// TODO bekijk of we toch echt zowel de lexeme als de literal nodig? Hebben we ze uberhoupt nodig?
// Kan ik niet gewoon hun positie in de source code meegeven en dan ze interpreteren (op basis van
// token type) of subslicen indien ik het relevante source code gedeelete nodig heb? Ik denk dat ik
// eens een talk van Zig had gezien of Data driven development waar Andrew Kelly sprak over het
// versnellen van de compiler en het reduceren van de token code.
// source: Andrew Kelley: A Practical Guide to Applying Data Oriented Design (DoD)
// https://www.youtube.com/watch?v=IroPQ150F6c
// Bekijk na implementeren van de interpreter en zie of ik het sneller kan maken!
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Literal>, line: u32) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
}

// Optional: implement Display for nicer printing
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {{ type: {:?}, lexeme: '{}', literal: '{:?}', line: {} }}",
            self.ttype, self.lexeme, self.literal, self.line
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
