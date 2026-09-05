use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;

use crate::backend::value::LoxValue;

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone)]
pub struct GlobalEnvironment {
    values: FxHashMap<String, LoxValue>,
}

impl GlobalEnvironment {
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: FxHashMap::default(),
        }
    }

    pub fn define_global_value(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn assign_global_value(&mut self, name: &str, value: LoxValue) -> Result<()> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            return Ok(());
        }
        anyhow::bail!("Undefined variable {name}");
    }

    pub fn get_global_value(&self, name: &str) -> Result<LoxValue> {
        // First try to access the inners scope
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }

        anyhow::bail!("Undefined variable {name}");
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    values: Vec<LoxValue>,
    enclosing: Option<Env>,
}

impl Environment {
    #[must_use]
    pub fn new() -> Env {
        Rc::new(RefCell::new(Self {
            values: Vec::new(),
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
        Rc::new(RefCell::new(Self {
            values: Vec::new(),
            enclosing: Some(enclosing),
        }))
    }

    pub fn define(&mut self, value: LoxValue) {
        self.values.push(value);
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

    pub fn assign_at(
        env: &Env,
        depth: usize,
        slot: usize,
        name: &str,
        value: LoxValue,
    ) -> Result<()> {
        let ancestor = Self::ancestor(env, depth)?;
        let mut ancestor = ancestor.borrow_mut();

        let target: &mut LoxValue = ancestor
            .values
            .get_mut(slot)
            .ok_or_else(|| anyhow::anyhow!("Invalid local slot"))?;
        *target = value;

        Ok(())
    }

    pub fn get_at(env: &Env, depth: usize, slot: usize, name: &str) -> Result<LoxValue> {
        let ancestor = Self::ancestor(env, depth)?;
        let ancestor = ancestor.borrow();

        ancestor
            .values
            .get(slot)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Undefined variable {name}"))
    }

    // pub fn assign(&mut self, name: &str, value: LoxValue) -> Result<()> {
    //     if self.values.contains_key(name) {
    //         self.values.insert(name.to_string(), value);
    //         return Ok(());
    //     }
    //     if let Some(enclosing) = &self.enclosing {
    //         enclosing.borrow_mut().assign(name, value)?;
    //         return Ok(());
    //     }

    //     anyhow::bail!("Undefined variable {name}");
    // }

    // pub fn get(&self, name: &str) -> Result<LoxValue> {
    //     // First try to access the inners scope
    //     if let Some(value) = self.values.get(name) {
    //         return Ok(value.clone());
    //     }
    //     // Match on Some if present
    //     if let Some(enclosing) = &self.enclosing {
    //         return enclosing.borrow().get(name);
    //     }

    //     anyhow::bail!("Undefined variable {name}");
    // }
}
