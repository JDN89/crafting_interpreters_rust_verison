use core::fmt;
use std::fmt::write;

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
    pub line: u32,
}

// TODO remove lexeme (strings are or can be expensive to clone "")
impl Token {
    pub fn new(ttype: TokenType, lexeme: String, line: u32) -> Token {
        Token {
            ttype,
            lexeme,
            line,
        }
    }
}

// Optional: implement Display for nicer printing
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {{ type: {:?}, lexeme: '{}',  line: {} }}",
            self.ttype, self.lexeme, self.line
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "TT: LeftParen"),
            TokenType::RightParen => write!(f, "TT: RightParen"),
            TokenType::LeftBrace => write!(f, "TT: LeftBrace"),
            TokenType::RightBrace => write!(f, "TT: RightBrace"),
            TokenType::Comma => write!(f, "TT: Comma"),
            TokenType::Dot => write!(f, "TT: Dot"),
            TokenType::Minus => write!(f, "TT: Minus"),
            TokenType::Plus => write!(f, "TT: Plus"),
            TokenType::Semicolon => write!(f, "TT: Semicolon"),
            TokenType::Slash => write!(f, "TT: Slash"),
            TokenType::Star => write!(f, "TT: Star"),
            TokenType::Bang => write!(f, "TT: Bang"),
            TokenType::BangEqual => write!(f, "TT: BangEqual"),
            TokenType::Equal => write!(f, "TT: Equal"),
            TokenType::EqualEqual => write!(f, "TT: EqualEqual"),
            TokenType::Greater => write!(f, "TT: Greater"),
            TokenType::GreaterEqual => write!(f, "TT: GreaterEqual"),
            TokenType::Less => write!(f, "TT: Less"),
            TokenType::LessEqual => write!(f, "TT: LessEqual"),
            TokenType::Identifier => write!(f, "TT: Identifier"),
            TokenType::String => write!(f, "TT: String"),
            TokenType::Number => write!(f, "TT: Number"),
            TokenType::And => write!(f, "TT: And"),
            TokenType::Class => write!(f, "TT: Class"),
            TokenType::Else => write!(f, "TT: Else"),
            TokenType::False => write!(f, "TT: False"),
            TokenType::Fun => write!(f, "TT: Fun"),
            TokenType::For => write!(f, "TT: For"),
            TokenType::If => write!(f, "TT: If"),
            TokenType::Nil => write!(f, "TT: Nil"),
            TokenType::Or => write!(f, "TT: Or"),
            TokenType::Print => write!(f, "TT: Print"),
            TokenType::Return => write!(f, "TT: Return"),
            TokenType::Super => write!(f, "TT: Super"),
            TokenType::This => write!(f, "TT: This"),
            TokenType::True => write!(f, "TT: True"),
            TokenType::Var => write!(f, "TT: Var"),
            TokenType::While => write!(f, "TT: While"),
            TokenType::Eof => write!(f, "TT: Eof"),
        }
    }
}
