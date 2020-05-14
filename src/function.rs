use crate::chunk::ChunkIndex;
use crate::error::RuntimeError;
use crate::memory::Value;
use std::hash::Hash;
use string_interner::Sym;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Function {
    pub name: Sym,
    pub chunk_index: ChunkIndex,
    pub arity: usize,
}

impl From<&Function> for Function {
    fn from(value: &Function) -> Self {
        Function {
            name: value.name.clone(),
            chunk_index: value.chunk_index,
            arity: value.arity,
        }
    }
}

pub struct NativeFunction {
    pub name: Sym,
    pub arity: usize,
    pub code: fn(&[Value]) -> Result<Value, RuntimeError>,
}

impl std::fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

impl PartialEq<NativeFunction> for NativeFunction {
    fn eq(&self, other: &NativeFunction) -> bool {
        self.name == other.name
    }
}

impl Eq for NativeFunction {}
