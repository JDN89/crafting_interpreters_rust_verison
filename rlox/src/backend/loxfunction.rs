use crate::{
    backend::{callable::LoxCallable, environment::{Env, Environment}, exec_signal::ExecSignal},
    frontend::ast::Stmt,
};

use super::value::LoxValue;

/*  NOTE: Each function call gets its own environment.
 Otherwise, recursion would break. If there are multiple calls to the same function in play at the same time,
 each needs its own environment, even though they are all calls to the same function.

 At creation time we capture the current environment and store it inside the loxfunction
 At call time we create a new environemnt whose parent is the captured closure. Then when we execute the loxfunction block when we encounter an Expr::Variable we first check if the variable is present in the current environment and if not we check if it is present in the parent environemnt
*/

//TODO for Closures this will probably change to at declaration time
#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub declaration: Stmt,
    pub closure: Env,
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        let Stmt::Function { params, .. } = &self.declaration else {
            unreachable!("LoxFunction can only contain a Stmt::LoxFunction")
        };
        params.len()
    }

    fn call(
        &self,
        interpreter: &mut super::interpreter::Interpreter,
        arguments: Vec<LoxValue>,
    ) -> anyhow::Result<LoxValue> {

        let env = Environment::new_enclosed(self.closure.clone());

        let Stmt::Function { params, body, .. } = &self.declaration else {
            unreachable!("LoxFunction can only contain a Stmt::LoxFunction")
        };

        // bind params to arguments and store then in the local enviroment
        for (arg, param) in arguments.into_iter().zip(params) {
            env.borrow_mut().define(param.lexeme.clone(), arg);
        }

        // evaluate the function block and return the value to who needs it.
        // That value then gets used by the surrounding code, for example:
        // - print foo(); prints it
        // - var x = foo(); stores it
        // - bar(foo()); passes it as an argument
        // - return foo(); returns it again from the outer function
        match interpreter.execute_block(body, env)? {
            ExecSignal::Normal => Ok(LoxValue::Nil),
            ExecSignal::Return(value) => Ok(value),
        }
    }
}

impl std::fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Stmt::Function { name, .. } = &self.declaration {
            write!(f, "<fn {}>", name.lexeme)
        } else {
            write!(f, "<fn>")
        }
    }
}
