/// Defines a runtime implementation of 'Class', 'BoundMethod', and 'Instance'
/// to use during program execution.
///
/// The runtime implementation of these structs keep track of more state than their
/// compile-time counterparts, and as such it is more efficient to redefine their
/// new states here.
use crate::vm::function::Closure;
use crate::vm::gc::managed::Gc;
use crate::vm::gc::managed::Trace;
use crate::vm::memory::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use string_interner::Sym;

#[derive(Debug, PartialEq)]
pub struct Class {
    pub name: Sym,
    pub methods: HashMap<Sym, Gc<Closure>>,
}

impl Trace for Class {
    fn trace(&self) {
        self.methods.trace();
    }
}

#[derive(Debug, PartialEq)]
pub struct Instance {
    pub class: Gc<RefCell<Class>>,
    pub fields: HashMap<Sym, Value>,
}

impl Trace for Instance {
    fn trace(&self) {
        self.class.trace();
        self.fields.trace();
    }
}

#[derive(Debug)]
pub struct BoundMethod {
    pub receiver: Gc<RefCell<Value>>,
    pub method: Gc<Closure>,
}

impl Trace for BoundMethod {
    fn trace(&self) {
        self.receiver.trace();
        self.method.trace();
    }
}
