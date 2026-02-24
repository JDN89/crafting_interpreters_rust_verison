use core::fmt;

use crate::frontend::ast::Literal;

// The evaluated Lox value. I was first reusing the Ast.Literal, but this would cause troubles if I
// have to exten my runtime values,...
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Boolean(bool),
    Float(f64),
    Nil,
}

// https://doc.rust-lang.org/std/convert/trait.From.html
impl From<Literal> for Value {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Str(s) => Value::Str(s),
            Literal::Boolean(b) => Value::Boolean(b),
            Literal::Float(n) => Value::Float(n),
            Literal::Nil => Value::Nil,
        }
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{s}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::Nil => write!(f, "nill"),
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
