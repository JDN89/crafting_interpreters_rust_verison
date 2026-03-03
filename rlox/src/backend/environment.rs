use std::collections::HashMap;

use anyhow::Ok;
use anyhow::Result;

use crate::backend::value::LoxValue;

pub struct Environment {
    values: HashMap<String, LoxValue>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: &str) -> Result<LoxValue> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }

        anyhow::bail!("Undefined variable {}", name);
    }
}
