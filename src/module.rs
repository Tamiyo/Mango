use crate::chunk::Chunk;
use crate::chunk::ChunkIndex;
use crate::chunk::ConstantIndex;
use crate::chunk::Instruction;
use crate::constant::Constant;
use crate::memory::ConstantPool;
use string_interner::StringInterner;
use string_interner::Sym;
#[derive(Debug, Clone)]
pub struct Module {
    pub chunks: Vec<Chunk>,
    pub constants: ConstantPool,
    pub strings: StringInterner<Sym>,
}

impl Module {
    pub fn new(strings: StringInterner<Sym>) -> Self {
        Module {
            chunks: vec![],
            constants: ConstantPool::new(),
            strings,
        }
    }

    pub fn add_chunk(&mut self) -> ChunkIndex {
        self.chunks.push(Chunk::new());
        self.chunks.len() - 1
    }

    pub fn get_chunk(&self, chunk_index: usize) -> &Chunk {
        &self.chunks[chunk_index]
    }

    pub fn get_chunk_mut(&mut self, chunk_index: usize) -> &mut Chunk {
        &mut self.chunks[chunk_index]
    }

    pub fn add_constant(&mut self, constant: Constant) -> ConstantIndex {
        self.constants.add(constant)
    }

    pub fn get_or_intern(&mut self, name: &str) -> Sym {
        self.strings.get_or_intern(name)
    }

    pub fn constants(&self) -> &ConstantPool {
        &self.constants
    }
}
