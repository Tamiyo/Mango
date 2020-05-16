use crate::vm::function::Function;
use crate::vm::gc::Trace;
use crate::vm::managed::Managed;
use crate::vm::memory::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use string_interner::Sym;

#[derive(Debug, PartialEq)]
pub struct Class {
    pub name: Sym,
    pub methods: HashMap<Sym, Managed<Function>>,
}

impl Trace for Class {
    fn trace(&self) {}
}

#[derive(Debug, PartialEq)]
pub struct Instance {
    pub class: Managed<RefCell<Class>>,
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
    pub receiver: Value,
    pub method: Managed<Function>,
}

impl Trace for BoundMethod {
    fn trace(&self) {
        self.receiver.trace();
        self.method.trace();
    }
}
