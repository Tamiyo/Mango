use crate::chunk::ChunkIndex;

pub struct Function {
    pub(crate) name: String,
    pub(crate) chunk_index: ChunkIndex,
    pub(crate) arity: usize,
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