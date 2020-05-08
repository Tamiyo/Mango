use crate::chunk::Chunk;
use crate::chunk::ChunkIndex;
use crate::chunk::ConstantIndex;
use crate::chunk::Instruction;
use crate::constant::Constant;
use crate::memory::ConstantPool;
use string_interner::StringInterner;
use string_interner::Sym;
<<<<<<< HEAD
#[derive(Debug, Clone)]
pub struct Module {
    pub chunks: Vec<Chunk>,
    pub constants: ConstantPool,
    pub strings: StringInterner<Sym>,
}

impl Module {
    pub fn new(strings: StringInterner<Sym>) -> Self {
=======
#[derive(Debug)]
pub struct Module<'a> {
    pub chunks: Vec<Chunk>,
    pub constants: ConstantPool,
    pub strings: &'a StringInterner<Sym>,
}

impl<'a> Module<'a> {
    pub fn new(strings: &'a StringInterner<Sym>) -> Self {
>>>>>>> 382353fd91b0585622e95c4ebfd4e877abef4353
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
<<<<<<< HEAD
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
=======
    }

    pub fn get_chunk_mut(&mut self, chunk_index: usize) -> &mut Chunk {
        &mut self.chunks[chunk_index]
    }

    pub fn add_constant(&mut self, constant: Constant) -> ConstantIndex {
        self.constants.add(constant)
>>>>>>> 382353fd91b0585622e95c4ebfd4e877abef4353
    }
}
