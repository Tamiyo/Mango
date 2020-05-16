use crate::bytecode::chunk::Chunk;
use crate::bytecode::chunk::ChunkIndex;
use crate::bytecode::chunk::ConstantIndex;
use crate::bytecode::constant::Constant;
use std::collections::HashMap;
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

    pub fn get_constant(&self, index: usize) -> &Constant {
        self.constants.get(index)
    }
}

#[derive(Debug, Clone)]
pub struct ConstantPool {
    pub pool: Vec<Constant>,
    pub cache: HashMap<Constant, usize>,
}

impl ConstantPool {
    pub fn new() -> Self {
        ConstantPool {
            pool: vec![],
            cache: HashMap::<Constant, usize>::new(),
        }
    }

    pub fn add(&mut self, constant: Constant) -> usize {
        if self.cache.contains_key(&constant) {
            *self.cache.get(&constant).unwrap()
        } else {
            self.cache.insert(constant.clone(), self.pool.len());
            self.pool.push(constant);
            self.pool.len() - 1
        }
    }

    pub fn get(&self, constant_index: usize) -> &Constant {
        &self.pool[constant_index]
    }
}
