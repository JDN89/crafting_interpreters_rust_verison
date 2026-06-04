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
            LoxValue::Str(s) => write!(f, "{:?}", s),
            LoxValue::Boolean(b) => write!(f, "{:?}", b),
            LoxValue::Float(n) => write!(f, "{:?}", n),
            LoxValue::Nil => write!(f, "Nil"),
            LoxValue::Callable(_) => write!(f, "<callable>"),
        }
    }
}

impl PartialEq for LoxValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LoxValue::Str(a), LoxValue::Str(b)) => a == b,
            (LoxValue::Boolean(a), LoxValue::Boolean(b)) => a == b,
            (LoxValue::Float(a), LoxValue::Float(b)) => a == b,
            (LoxValue::Nil, LoxValue::Nil) => true,
            (LoxValue::Callable(a), LoxValue::Callable(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
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
            LoxValue::Callable(_) => write!(f, "<callable>"),
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
