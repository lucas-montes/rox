use std::{collections::HashMap, ops::{Deref, DerefMut}};

use crate::syntax_tree::Literal;

#[derive(Default)]
pub struct Environment<'a>(HashMap<&'a str, Literal<'a>>);

impl<'a> Deref for Environment<'a> {
    type Target = HashMap<&'a str, Literal<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Environment<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
