use std::ops::{Deref, DerefMut};

pub enum Opcode<T> {
    Value(T),
    OpReturn, // Return from the current function
}

pub struct Chunk<T>(Vec<Opcode<T>>);

impl<T> Deref for Chunk<T> {
    type Target = Vec<Opcode<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Chunk<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
