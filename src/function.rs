use crate::chunk::ChunkIndex;
use crate::local::Upvalue;
use string_interner::Sym;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Closure {
<<<<<<< HEAD
    pub function: Function,
    pub upvalues: Vec<Upvalue>,
=======
    function: Function,
    upvalues: Vec<Upvalue>,
>>>>>>> 382353fd91b0585622e95c4ebfd4e877abef4353
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
<<<<<<< HEAD
    pub name: Sym,
    pub chunk_index: ChunkIndex,
    pub arity: usize,
=======
    name: Sym,
    chunk_index: ChunkIndex,
    arity: usize,
>>>>>>> 382353fd91b0585622e95c4ebfd4e877abef4353
}
