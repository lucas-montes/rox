pub enum Opcode {
    OpReturn, // Return from the current function
}

pub struct Chunk<T>(Vec<T>);
