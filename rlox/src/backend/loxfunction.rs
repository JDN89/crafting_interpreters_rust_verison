use crate::{
    backend::{callable::LoxCallable, environment::Environment},
    frontend::ast::Stmt,
};

use super::value::LoxValue;

// TODO can't i just store name, params and body here as well and pass the Stmt another way instead
// of storing Stmts ast inside LoxFunction which is a runtime wrapper that will be execued at runtime?
// TODO store Env as well here (the enclosing so the parent env?)
#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: Stmt,
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        todo!()
    }

    fn call(
        &self,
        interpreter: &mut super::interpreter::Interpreter,
        arguments: Vec<LoxValue>,
    ) -> anyhow::Result<LoxValue> {
        let mut env = Environment::new_enclosed(interpreter.environment.clone());

        // TODO do we need to clone here?
        let Stmt::Function { name, params, body } = self.declaration.clone() else {
            unreachable!("LoxFunction can only contain a Stmt::LoxFunction")
        };

        for (arg, param) in arguments.into_iter().zip(params) {
            env.get_mut().define(param.lexeme, arg);
        }

        interpreter.execute_block(&[body], env);

        return Ok(LoxValue::Nil);
    }
}
