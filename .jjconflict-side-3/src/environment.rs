use std::{collections::HashMap, ops::{Deref, DerefMut}, rc::Rc};

use crate::syntax_tree::Literal;


#[derive(Default, Debug)]
pub struct InternalEnv(HashMap<Rc<str>, Literal>);

impl Deref for InternalEnv {
    type Target = HashMap<Rc<str>, Literal>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InternalEnv {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct Environment {
    scopes: Vec<InternalEnv>,
}

impl Default for Environment {
    fn default() -> Self {
        Self { scopes: vec![InternalEnv::default()] }
    }
}

impl Environment {
    pub fn define(&mut self, key: &str, value: Literal) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(key.into(), value);
        }
    }

    pub fn get(&self, key: &str) -> Option<&Literal> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(value);
            }
        }
        None
    }

    pub fn assing(&mut self, key: &str, value: Literal) -> Option<Literal> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(key) {
                scope.insert(key.into(), value.clone());
                return Some(value);
            }
        }
        None
    }

     pub fn push_scope(&mut self) {
        self.scopes.push(InternalEnv::default());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
}
