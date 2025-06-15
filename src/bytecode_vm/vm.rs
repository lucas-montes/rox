use super::{chunk::Chunk, values::Value};

const STACK_MAX: usize = 256;

pub struct Vm {
    //TODO: we'll need either pointers, usize to point to the location or refs
    chunk: Chunk<Value>,
    ip: u8,                            // NOTE: this is a pointer to the values in the chunk
    stack: Vec<Value> //TODO: use maybeuninit
}

impl Vm {
    pub fn new(chunk: Chunk<Value>) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
        }
    }
    fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }
    fn push(&mut self, value: Value) {
        self.stack.push(value)
    }
    fn run(&mut self){}
    pub fn interpret(&mut self, chunk: Chunk<Value>){}
}
