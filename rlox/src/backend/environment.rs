use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use anyhow::Result;

use crate::backend::value::LoxValue;

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Option<Env>,
}

// impl Default for Environment {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Environment {
    pub fn new() -> Env {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: None,
        }))
    }

    /// Creates a new environment enclosed by `enclosing`, which is usually
    /// the parent or outer scope.
    ///
    /// This is the scope-chain link used for nested blocks and functions: the
    /// new environment starts empty, but any lookup or assignment that cannot
    /// be satisfied locally will fall back to the enclosing environment.
    pub fn new_enclosed(enclosing: Env) -> Env {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }))
    }

    // does the lifetime have to persist after this?
    // No, we are in the interpreter. we store the name and value in the environment so as
    // to keep track of it for the duration of the program. But we don't need to keep it around after
    // the function returns, so no clone is needed.
    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    fn ancestor(env: &Env, distance: usize) -> Result<Env> {
        let mut current = env.clone();

        for _ in 0..distance {
            let enclosing = current
                .borrow()
                .enclosing
                .clone()
                .ok_or_else(|| anyhow::anyhow!("Missing enclosing environment."))?;
            current = enclosing;
        }

        Ok(current)
    }

    pub fn assign_at(env: &Env, distance: usize, name: &str, value: LoxValue) -> Result<()> {
        let ancestor = Self::ancestor(env, distance)?;
        let mut ancestor = ancestor.borrow_mut();

        if ancestor.values.contains_key(name) {
            ancestor.values.insert(name.to_string(), value);
            return Ok(());
        }

        anyhow::bail!("Undefined variable {}", name);
    }

    pub fn get_at(env: &Env, distance: usize, name: &str) -> Result<LoxValue> {
        let ancestor = Self::ancestor(env, distance)?;
        let ancestor = ancestor.borrow();

        ancestor
            .values
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Undefined variable {}", name))
    }

    pub fn assign(&mut self, name: &str, value: LoxValue) -> Result<()> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value.clone());
            return Ok(());
        }
        if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)?;
            return Ok(());
        }

        anyhow::bail!("Undefined variable {}", name);
    }

    pub fn get(&self, name: &str) -> Result<LoxValue> {
        // First try to access the inners scope
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }
        // Match on Some if present
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }

        anyhow::bail!("Undefined variable {}", name);
    }
}
