use core::fmt;

use crate::frontend::ast::Literal;

// The evaluated Lox value. I was first reusing the Ast.Literal, but this would cause troubles if I
// have to exten my runtime values,...
#[derive(Debug, Clone, PartialEq)]
pub enum LoxValue {
    Str(String),
    Boolean(bool),
    Float(f64),
    Nil,
}

// https://doc.rust-lang.org/std/convert/trait.From.html
impl From<Literal> for LoxValue {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Str(s) => LoxValue::Str(s),
            Literal::Boolean(b) => LoxValue::Boolean(b),
            Literal::Float(n) => LoxValue::Float(n),
            Literal::Nil => LoxValue::Nil,
        }
    }
}
impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxValue::Str(s) => write!(f, "{s}"),
            LoxValue::Boolean(b) => write!(f, "{b}"),
            LoxValue::Float(n) => write!(f, "{n}"),
            LoxValue::Nil => write!(f, "nill"),
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime error: {}", self.message)
    }
}

impl std::error::Error for RuntimeError {}
