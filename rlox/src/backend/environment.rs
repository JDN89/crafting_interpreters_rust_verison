use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::rc::Rc;

use anyhow::Ok;
use anyhow::Result;

use crate::backend::value::LoxValue;

pub type EnvStack = Vec<Environment>;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Option<usize>,
}

// impl Default for Environment {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Environment {
    pub fn new() -> EnvStack {
        vec![Environment {
            values: HashMap::new(),
            enclosing: None,
        }]
    }

    pub fn new_enclosed(envs: &mut EnvStack, parent: usize) -> usize {
        envs.push(Environment {
            values: HashMap::new(),
            enclosing: Some(parent),
        });
        envs.len() - 1 // return index of new Environment
    }

    pub fn define(envs: &mut EnvStack, curr: usize, name: String, value: LoxValue) {
        envs[curr].values.insert(name, value);
    }

    pub fn assign(envs: &mut EnvStack, mut curr: usize, name: &str, value: LoxValue) -> Result<()> {
        loop {
            // loop over the environemts. if the current environment doesn't contain the key.
            // You go to the next one and keep looping until you find the key or there is no
            // parent environment
            if envs[curr].values.contains_key(name) {
                envs[curr].values.insert(name.to_string(), value);
                return Ok(());
            }
            match envs[curr].enclosing {
                Some(parent) => curr = parent, //parent found re-execute the loop
                None => break,                 // no parent environemnt found
            }
        }
        anyhow::bail!("Undefined variable {}", name);
    }

    // TODO refactor get function
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
