use core::fmt;

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

// TODO remove lexeme (strings are or can be expensive to clone ""). This is getting rediculous!
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
            Self::LeftParen => write!(f, "TT: LeftParen"),
            Self::RightParen => write!(f, "TT: RightParen"),
            Self::LeftBrace => write!(f, "TT: LeftBrace"),
            Self::RightBrace => write!(f, "TT: RightBrace"),
            Self::Comma => write!(f, "TT: Comma"),
            Self::Dot => write!(f, "TT: Dot"),
            Self::Minus => write!(f, "TT: Minus"),
            Self::Plus => write!(f, "TT: Plus"),
            Self::Semicolon => write!(f, "TT: Semicolon"),
            Self::Slash => write!(f, "TT: Slash"),
            Self::Star => write!(f, "TT: Star"),
            Self::Bang => write!(f, "TT: Bang"),
            Self::BangEqual => write!(f, "TT: BangEqual"),
            Self::Equal => write!(f, "TT: Equal"),
            Self::EqualEqual => write!(f, "TT: EqualEqual"),
            Self::Greater => write!(f, "TT: Greater"),
            Self::GreaterEqual => write!(f, "TT: GreaterEqual"),
            Self::Less => write!(f, "TT: Less"),
            Self::LessEqual => write!(f, "TT: LessEqual"),
            Self::Identifier => write!(f, "TT: Identifier"),
            Self::String => write!(f, "TT: String"),
            Self::Number => write!(f, "TT: Number"),
            Self::And => write!(f, "TT: And"),
            Self::Class => write!(f, "TT: Class"),
            Self::Else => write!(f, "TT: Else"),
            Self::False => write!(f, "TT: False"),
            Self::Fun => write!(f, "TT: Fun"),
            Self::For => write!(f, "TT: For"),
            Self::If => write!(f, "TT: If"),
            Self::Nil => write!(f, "TT: Nil"),
            Self::Or => write!(f, "TT: Or"),
            Self::Print => write!(f, "TT: Print"),
            Self::Return => write!(f, "TT: Return"),
            Self::Super => write!(f, "TT: Super"),
            Self::This => write!(f, "TT: This"),
            Self::True => write!(f, "TT: True"),
            Self::Var => write!(f, "TT: Var"),
            Self::While => write!(f, "TT: While"),
            Self::Eof => write!(f, "TT: Eof"),
        }
    }
}
