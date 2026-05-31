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

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Operator {
    Plus = 0,
    Minus = 1,
    Star = 2,
    Slash = 3,
    Equal = 4,
    Bang = 5,
    Greater = 6,
    GreaterEqual = 7,
    Less = 8,
    LessEqual = 9,
    BangEqual = 10,
    EqualEqual = 11,
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
    Logical {
        left: Box<Expr>,
        op: TokenType,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: TokenType,
        arguments: Vec<Expr>,
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
            Expr::Logical { left, op, right } => {
                write!(f, "({} {} {})", op, left, right)
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                write!(f, "({},{},{:#?})", callee, paren, arguments)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    IfStatement {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
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
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}
