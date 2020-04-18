use crate::chunk::{Chunk, ConstantIndex, ChunkIndex, Instruction};
use crate::constant::Constant;

pub struct Module {
    chunks: Vec<Chunk>,
    constants: Vec<Constant>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            chunks: vec![],
            constants: vec![],
        }
    }

    pub fn add_chunk(&mut self) -> ChunkIndex {
        self.chunks.push(Chunk::new());
        self.chunks.len() - 1
    }

    pub fn add_constant(&mut self, constant: Constant) -> ConstantIndex {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    pub fn add_instruction(&mut self, index: usize, instruction: Instruction) {
        self.chunks[index].add_instruction(instruction);
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }
}