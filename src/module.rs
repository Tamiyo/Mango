use crate::chunk::{Chunk, ConstantIndex, ChunkIndex, Instruction, InstructionIndex};
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

    pub fn add_instruction(&mut self, index: usize, instruction: Instruction) -> InstructionIndex {
        self.chunks[index].add_instruction(instruction)
    }

    pub fn chunk(&self, index: ChunkIndex) -> &Chunk {
        &self.chunks[index]
    }

    pub fn chunk_mut(&mut self, index: ChunkIndex) -> &mut Chunk {
        &mut self.chunks[index]
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }

    pub fn constant(&self, index: usize) -> Constant {
        self.constants[index].clone()
    }
}