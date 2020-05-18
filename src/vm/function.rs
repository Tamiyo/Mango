/// Defines a runtime implementation of 'Function', 'Closure', and 'NativeFunction'
/// to use during program execution.
///
/// The runtime implementation of these structs keep track of more state than their
/// compile-time counterparts, and as such it is more efficient to redefine their
/// new states here.
use crate::bytecode::chunk::ChunkIndex;
use crate::vm::error::RuntimeError;
use crate::vm::gc::managed::Gc;
use crate::vm::gc::managed::Trace;
use crate::vm::memory::Upvalue;
use crate::vm::memory::Value;
use core::cell::RefCell;
use string_interner::Sym;

#[derive(PartialEq, Debug)]
pub struct Function {
    pub name: Sym,
    pub chunk_index: ChunkIndex,
    pub arity: usize,
}

impl Trace for Function {
    fn trace(&self) {}
}
#[derive(PartialEq)]
pub struct Closure {
    pub function: Gc<Function>,
    pub upvalues: Vec<Gc<RefCell<Upvalue>>>,
}

impl std::fmt::Debug for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Closure({:?})", self.function)
    }
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
