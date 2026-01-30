use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal<'a> {
    Str(&'a str),
    Boolean(bool),
    Integer(i32),
}

// TODO: Does the Token need to be the owner of the Lexeme, or can we point to the source code?
// lexeme: &str
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal<'a>>,
    pub line: u32,
}

impl Token<'_> {
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
impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {{ type: {:?}, lexeme: '{}', literal: '{:?}', line: {} }}",
            self.ttype, self.lexeme, self.literal, self.line
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
