use core::fmt;
use std::rc::Rc;

use crate::backend::callable::LoxCallable;
use crate::frontend::ast::Literal;

// The evaluated Lox value. I was first reusing the Ast.Literal, but this would cause troubles if I
// have to exten my runtime values,...
#[derive(Clone)]
pub enum LoxValue {
    Str(String),
    Boolean(bool),
    Float(f64),
    Nil,
    // TODO: replace dyn trait later with enum variant. more explicit
    Callable(Rc<dyn LoxCallable>),
}

impl fmt::Debug for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Str(s) => write!(f, "{s:?}"),
            Self::Boolean(b) => write!(f, "{b:?}"),
            Self::Float(n) => write!(f, "{n:?}"),
            Self::Nil => write!(f, "Nil"),
            Self::Callable(_) => write!(f, "<callable>"),
        }
    }
}

impl PartialEq for LoxValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Str(a), Self::Str(b)) => a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,
            (Self::Nil, Self::Nil) => true,
            (Self::Callable(a), Self::Callable(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

// https://doc.rust-lang.org/std/convert/trait.From.html
impl From<Literal> for LoxValue {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Str(s) => Self::Str(s),
            Literal::Boolean(b) => Self::Boolean(b),
            Literal::Float(n) => Self::Float(n),
            Literal::Nil => Self::Nil,
        }
    }
}
impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Str(s) => write!(f, "{s}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Float(n) => write!(f, "{n}"),
            Self::Nil => write!(f, "nill"),
            Self::Callable(_) => write!(f, "<callable>"),
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
