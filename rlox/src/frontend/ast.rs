use core::fmt;
use std::cell::Cell;

use anyhow::{Error, Ok, Result, anyhow};

use crate::frontend::token::{Token, TokenType};

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
            Self::Str(s) => write!(f, "{s}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Float(n) => write!(f, "{n}"),
            Self::Nil => write!(f, "nill"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
            TokenType::Plus => Ok(Self::Plus),
            TokenType::Minus => Ok(Self::Minus),
            TokenType::Star => Ok(Self::Star),
            TokenType::Slash => Ok(Self::Slash),
            TokenType::Bang => Ok(Self::Bang),
            TokenType::EqualEqual => Ok(Self::EqualEqual),
            TokenType::BangEqual => Ok(Self::BangEqual),
            TokenType::Less => Ok(Self::Less),
            TokenType::LessEqual => Ok(Self::LessEqual),
            TokenType::Greater => Ok(Self::Greater),
            TokenType::GreaterEqual => Ok(Self::GreaterEqual),
            TokenType::Equal => Ok(Self::Equal),
            _ => Err(anyhow!(
                "[TokenType {ttype:?}] doesn't have a matching operator"
            )),
        }
    }
}
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
            Self::Equal => "=",
            Self::Bang => "!",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::BangEqual => "!=",
            Self::EqualEqual => "==",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Logical {
        left: Box<Self>,
        op: TokenType,
        right: Box<Self>,
    },
    Binary {
        left: Box<Self>,
        op: Operator,
        right: Box<Self>,
    },
    Call {
        callee: Box<Self>,
        paren: TokenType,
        arguments: Vec<Self>,
    },
    Assign {
        name: String,
        value: Box<Self>,
        scope_depth: Cell<Option<usize>>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        op: Operator,
        right: Box<Self>,
    },
    Variable {
        name: String,
        scope_depth: Cell<Option<usize>>,
    },
    Grouping {
        value: Box<Self>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binary { left, op, right } => {
                write!(f, "({op} {left} {right})")
            }
            Self::Unary { op, right } => {
                write!(f, "({op} {right})")
            }
            Self::Grouping { value } => {
                write!(f, "(group {value})")
            }
            Self::Literal { value } => {
                write!(f, "{value}")
            }
            Self::Variable { name, scope_depth } => match scope_depth.get() {
                Some(depth) => write!(f, "{name}, depth = {depth}"),
                None => write!(f, "{name}"),
            },
            Self::Assign { name, value, .. } => {
                write!(f, "({name} {value})")
            }
            Self::Logical { left, op, right } => {
                write!(f, "({op} {left} {right})")
            }
            Self::Call {
                callee,
                paren,
                arguments,
            } => write!(f, "({callee}, {paren}, {arguments:?})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    IfStatement {
        condition: Expr,
        then_branch: Box<Self>,
        else_branch: Option<Box<Self>>,
    },
    ExpressionStmt {
        expr: Expr,
    },
    PrintStmt {
        expr: Expr,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    // NOTE: var declaration has optional initializer
    Var {
        name: String,
        initializer: Option<Expr>,
    },
    Block {
        statements: Vec<Self>,
    },
    While {
        condition: Expr,
        body: Box<Self>,
    },

    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Self>,
    },
}
