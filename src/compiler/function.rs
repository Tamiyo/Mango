use crate::bytecode::chunk::ChunkIndex;
use crate::compiler::local::Upvalue;
use string_interner::Sym;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]

pub struct Function {
    pub name: Sym,
    pub chunk_index: ChunkIndex,
    pub arity: usize,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Closure {
    pub function: Function,
    pub upvalues: Vec<Upvalue>,
}
