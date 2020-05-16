use crate::bytecode::chunk::ChunkIndex;
use crate::vm::error::RuntimeError;
use crate::vm::gc::Trace;
use crate::vm::managed::Managed;
use crate::vm::memory::Upvalue;
use crate::vm::memory::Value;
use core::cell::RefCell;
use string_interner::Sym;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Function {
    pub name: Sym,
    pub chunk_index: ChunkIndex,
    pub arity: usize,
}

impl Trace for Function {
    fn trace(&self) {}
}

#[derive(Debug, PartialEq, Clone)]
pub struct Closure {
    pub function: Managed<Function>,
    pub upvalues: Vec<Managed<RefCell<Upvalue>>>,
}

impl Trace for Closure {
    fn trace(&self) {
        self.function.trace();
        self.upvalues.trace();
    }
}
pub struct NativeFunction {
    pub name: Sym,
    pub arity: usize,
    pub code: fn(&[Value]) -> Result<Value, RuntimeError>,
}

impl Trace for NativeFunction {
    fn trace(&self) {}
}

impl std::fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {:?}", self.name, self.arity)
    }
}

impl PartialEq<NativeFunction> for NativeFunction {
    fn eq(&self, other: &NativeFunction) -> bool {
        self.name == other.name
    }
}

impl Eq for NativeFunction {}
