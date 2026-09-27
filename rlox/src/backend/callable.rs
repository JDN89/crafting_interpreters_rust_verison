use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;

use crate::backend::interpreter::Interpreter;
use crate::backend::value::LoxValue;
use crate::frontend::ast::Ast;

// Native functions always implement these methods
// alternative is defining an enum that contains all native funtions, match and execute the logic
pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxValue>,
        ast: &Ast,
    ) -> Result<LoxValue>;
}

#[derive(Debug, Clone, Copy)]
pub struct Clock;

impl LoxCallable for Clock {
    fn arity(&self) -> usize {
        0
    }

    // TODO when I turn this into an enum maybe I can remove this unused ast arg?
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<LoxValue>,
        _ast: &Ast,
    ) -> Result<LoxValue> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|err| anyhow::anyhow!("system clock is before UNIX_EPOCH: {err}"))?;

        Ok(LoxValue::Float(duration.as_secs_f64()))
    }
}
