use crate::{
    frontend::{lox_callable::LoxCallable, lox_value::LoxValue},
    tree_walker::environment::Environment,
    LoxError,
};
use std::rc::Rc;

use super::{interpreter::Interpreter, parser::FunctionDecl};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub declaration: FunctionDecl,
}

impl LoxFunction {
    pub fn new(declaration: FunctionDecl) -> Self {
        Self { declaration }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<LoxValue>,
    ) -> Result<LoxValue, LoxError> {
        // println!("execute call -> mutating global environemnts?");
        // NOTE: instead of getting access to the environemt I made a copy of the pointer to the
        // parent environment. Which broke recursion because we kept mutating the global
        // environment instead of creating a scope for each function call.
        // SEE chapter 10 -> 10.4 Function calls
        // Further, this environment must be created dynamically. Each function call gets its own environment.
        // Otherwise, recursion would break.
        // If there are multiple calls to the same function in play at the same time, each needs its own environment,
        // even though they are all calls to the same function.
        //
        //
        //
        // NOTE: we new inner env has pointer to globals!!! ERROR found?
        // OLD code block -> error was Rc::clone
        // let mut env = Environment::new_inner_environment(Rc::clone(&interpreter.globals));
        // for (parameter, value) in self.declaration.parameters.iter().zip(args.iter()) {
        //     env.define(&parameter.clone().lexeme, value.clone());
        // }
        //with each function call we create a new code env
        //pass the env to execute code block where the code gets executed with the vars bounded to
        //the environement.
        //the parent env gets restored after the function environment has been interpreted
        // NOTE: we new inner env has pointer to globals!!! ERROR found?
        let mut env = Environment::new_inner_environment(Rc::clone(&interpreter.globals));
        // println!(" \n environment variables: {:?} \n", env);
        for (parameter, value) in self.declaration.parameters.iter().zip(args.iter()) {
            env.define(&parameter.clone().lexeme, value.clone());
        }

        let result = interpreter.execute_block(&self.declaration.body, env);

        // TODO: I don't like that we wrap the return value in an error!!
        match result {
            Ok(()) => Ok(LoxValue::Nil),
            Err(LoxError::Return(value)) => Ok(value),
            Err(e) => Err(e),
        }
    }

    fn name(&self) -> &str {
        &self.declaration.name.lexeme
    }
}
