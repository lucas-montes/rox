use std::{collections::HashMap, rc::Rc};

use crate::syntax_tree::Literal;

#[derive(Default, Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    inner: HashMap<Rc<str>, Literal>,
}

impl Environment {
    pub fn define(&mut self, key: &str, value: Literal) {
        self.inner.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Literal> {
        self.inner
            .get(key)
            .or(self.enclosing.as_ref().and_then(|e| (*e).get(key)))
    }

    pub fn assing(&mut self, key: &str, value: Literal) -> Option<Literal> {
        if self.inner.contains_key(key) {
            self.inner.insert(key.into(), value.clone());
            Some(value)
        } else {
            self.enclosing
                .as_mut()
                .and_then(|e| (*e).assing(key, value))
        }
    }

    pub fn set_enclosing(&mut self, env: Environment) {
        self.enclosing = Some(Box::new(env));
    }
}
