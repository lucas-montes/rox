use super::{chunk::Chunk, values::Value};

const STACK_MAX: usize = 256;

pub struct Vm {
    //TODO: we'll need either pointers, usize to point to the location or refs
    chunk: Chunk<Value>,
    ip: u8, // NOTE: this is a pointer to the values in the chunk
    stack: [Option<Value>; STACK_MAX], //TODO: use maybeuninit
    top_stack: Option<Value>,
}

impl Vm {
    pub fn new(chunk: Chunk<Value>) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: [None; STACK_MAX],
            top_stack: None,
        }
    }
}
