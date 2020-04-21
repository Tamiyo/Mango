use crate::chunk::ChunkIndex;

pub struct Function {
    name: String,
    chunk_index: ChunkIndex,
    arity: usize,
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