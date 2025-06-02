use std::{collections::HashMap, ops::{Deref, DerefMut}, rc::Rc};

use crate::syntax_tree::Literal;

#[derive(Default)]
pub struct Environment(HashMap<Rc<str>, Literal>);

impl Deref for Environment {
    type Target = HashMap<Rc<str>, Literal>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Environment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
