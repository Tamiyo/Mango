use crate::chunk::ChunkIndex;
use crate::constant::Constant;
use crate::error::RuntimeError;
use std::hash::Hash;
use std::hash::Hasher;
use string_interner::Sym;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Function {
    pub name: Sym,
    pub chunk_index: ChunkIndex,
    pub arity: usize,
}

#[derive(Clone)]
pub struct NativeFunction {
    pub sym: Sym,
    pub arity: usize,
    pub code: fn(&[Constant]) -> Result<Constant, RuntimeError>,
}

impl std::fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.sym)
    }
}

impl PartialEq<NativeFunction> for NativeFunction {
    fn eq(&self, other: &NativeFunction) -> bool {
        self.sym == other.sym
    }
}
impl Eq for NativeFunction {}

impl Hash for NativeFunction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sym.hash(state)
    }
}
