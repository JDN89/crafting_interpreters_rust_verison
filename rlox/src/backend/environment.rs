use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use anyhow::Ok;
use anyhow::Result;

use crate::backend::environment;
use crate::backend::value::LoxValue;

pub type Env = Rc<RefCell<Environment>>;

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
    pub fn new_enclosed(enclosing: Env) -> Env {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }))
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name.clone(), value.clone());

        if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().values.insert(name, value);
        }
    }
    pub fn get(&self, name: &str) -> Result<LoxValue> {
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
