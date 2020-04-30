use crate::chunk::ChunkIndex;
use crate::local::Upvalue;
use string_interner::Sym;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Closure {
    function: Function,
    upvalues: Vec<Upvalue>,
}

impl Closure {
    pub fn new(function: Function) -> Self {
        Closure {
            function,
            upvalues: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Function {
    name: Sym,
    chunk_index: ChunkIndex,
    arity: usize,
}
