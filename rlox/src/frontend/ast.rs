// TODO: I am doubting if I should immediatley use an allocator and push the expr on it, that
// returns an exprId. Binary
// would become
// {left: exprId, operator:..., right: ExprId}. This way I don't have to mess with Box, the expr
// won't be spread out in memory,... But I want to see the memory and performance gains, so lets do
// it the naive way first

use core::fmt;

use anyhow::*;

use crate::frontend::token::TokenType;

// TODO this enum is exactly the same as the token Literal enum. Something's got to go!!
// TODO even the source can go on the allocator. when we need a substring just store the start and
// end point as ints and when they are actually needed just index into the source code and extract
// the slice like that. I think for now I'll just clone the thing and see how much performance I
// gain by the above approach. I don't want to drag the lifetimes around
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Str(String),
    Boolean(bool),
    Float(f64),
    Nil,
}
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Str(s) => write!(f, "{s}"),
            Literal::Boolean(b) => write!(f, "{b}"),
            Literal::Float(n) => write!(f, "{n}"),
            Literal::Nil => write!(f, "nill"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Bang,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    BangEqual,
    EqualEqual,
}
impl Operator {
    pub fn from_token_type(ttype: TokenType) -> Result<Self, Error> {
        match ttype {
            TokenType::Plus => Ok(Operator::Plus),
            TokenType::Minus => Ok(Operator::Minus),
            TokenType::Star => Ok(Operator::Star),
            TokenType::Slash => Ok(Operator::Slash),
            TokenType::Bang => Ok(Operator::Bang),
            TokenType::EqualEqual => Ok(Operator::EqualEqual),
            TokenType::BangEqual => Ok(Operator::BangEqual),
            TokenType::Less => Ok(Operator::Less),
            TokenType::LessEqual => Ok(Operator::LessEqual),
            TokenType::Greater => Ok(Operator::Greater),
            TokenType::GreaterEqual => Ok(Operator::GreaterEqual),
            TokenType::Equal => Ok(Operator::Equal),
            _ => Err(anyhow!(
                "[TokenType {:?}] doesn't have a matching operator",
                ttype
            )),
        }
    }
}
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Star => "*",
            Operator::Slash => "/",
            Operator::Equal => "=",
            Operator::Bang => "!",
            Operator::Greater => ">",
            Operator::GreaterEqual => ">=",
            Operator::Less => "<",
            Operator::LessEqual => "<=",
            Operator::BangEqual => "!=",
            Operator::EqualEqual => "==",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Assign {
        name: String,
        value: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        op: Operator,
        right: Box<Expr>,
    },
    Variable {
        name: String,
    },
    Grouping {
        value: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary { left, op, right } => {
                write!(f, "({} {} {})", op, left, right)
            }

            Expr::Unary { op, right } => {
                write!(f, "({} {})", op, right)
            }

            Expr::Grouping { value } => {
                write!(f, "(group {})", value)
            }

            Expr::Literal { value } => {
                write!(f, "{value}")
            }

            Expr::Variable { name } => {
                write!(f, "{name}")
            }

            Expr::Assign { name, value } => {
                write!(f, "({} {})", name, value)
            }
        }
    }
}

pub enum Stmt {
    ExpressionStmt {
        expr: Expr,
    },
    PrintStmt {
        expr: Expr,
    },
    // NOTE: variable declaration laten normaal gezien optional initializers to. var halo;
    Var {
        name: String,
        initializer: Option<Expr>,
    },
    Block {
        statements: Vec<Stmt>,
    },
}
