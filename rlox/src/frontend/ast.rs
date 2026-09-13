use core::fmt;
use std::cell::Cell;

use anyhow::{Error, Ok, Result, anyhow};

use crate::frontend::token::{Token, TokenType};

pub type Depth = usize;
pub type Slot = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(u32);

impl fmt::Display for ExprId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StmtId(u32);
impl fmt::Display for StmtId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub expressions: Vec<Expr>,
    pub statements: Vec<Stmt>,
}

// TODO find out if I can, based on number of tokes. create a fixed length Vecs?
impl Ast {
    pub const fn new() -> Self {
        Self {
            expressions: Vec::new(),
            statements: Vec::new(),
        }
    }
    pub fn push_expression(&mut self, expr: Expr) -> ExprId {
        let id = ExprId(self.expressions.len() as u32);
        self.expressions.push(expr);
        return id;
    }

    pub fn push_statement(&mut self, stmt: Stmt) -> StmtId {
        let id = StmtId(self.statements.len() as u32);
        self.statements.push(stmt);
        return id;
    }

    // TODO normaly .clone form function but see if reference is enough at the places where you call this
    pub fn get_expression(&self, id: ExprId) -> Option<&Expr> {
        self.expressions.get(id.0 as usize)
    }

    // TODO normaly .clone form function but see if reference is enough at the places where you call this
    pub fn get_statement(&self, id: StmtId) -> Option<&Stmt> {
        self.statements.get(id.0 as usize)
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        left: ExprId,
        op: TokenType,
        right: ExprId,
    },
    Binary {
        left: ExprId,
        op: Operator,
        right: ExprId,
    },
    Call {
        callee: ExprId,
        paren: TokenType,
        arguments: Vec<ExprId>,
    },
    Assign {
        name: String,
        value: ExprId,
        env_location: Cell<Option<(Depth, Slot)>>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        op: Operator,
        right: ExprId,
    },
    Variable {
        name: String,
        env_location: Cell<Option<(Depth, Slot)>>,
    },
    Grouping {
        value: ExprId,
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
            Self::Variable { name, env_location } => match env_location.get() {
                Some(location) => write!(f, "{name}, location= {location:?}"),
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
        condition: ExprId,
        then_branch: StmtId,
        else_branch: Option<StmtId>,
    },
    ExpressionStmt {
        expr: ExprId,
    },
    PrintStmt {
        expr: ExprId,
    },
    Return {
        keyword: Token,
        value: Option<ExprId>,
    },
    // NOTE: var declaration has optional initializer
    Var {
        name: String,
        initializer: Option<ExprId>,
        env_location: Cell<Option<(Depth, Slot)>>,
    },
    Block {
        statements: Vec<StmtId>,
    },
    While {
        condition: ExprId,
        body: StmtId,
    },

    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<StmtId>,
        env_location: Cell<Option<(Depth, Slot)>>,
    },
}
