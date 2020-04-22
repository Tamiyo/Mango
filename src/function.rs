use crate::chunk::ChunkIndex;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub chunk_index: ChunkIndex,
    pub arity: usize,
}

impl Function {
    pub fn new(name: String, chunk_index: ChunkIndex, arity: usize) -> Self {
        Function {
            name,
            chunk_index,
            arity,
        }
    }
}

pub struct Closure {
    function: Function,
    // upvalues: Vec<Upvalue>
}