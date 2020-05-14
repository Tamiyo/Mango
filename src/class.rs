use crate::function::Function;
use crate::managed::Managed;
use crate::memory::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use string_interner::Sym;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ConstantClass {
    pub name: Sym,
}

#[derive(Debug, PartialEq)]
pub struct Class {
    pub name: Sym,
    pub methods: HashMap<Sym, Managed<Function>>,
}

#[derive(Debug, PartialEq)]
pub struct Instance {
    pub class: Managed<RefCell<Class>>,
    pub fields: HashMap<Sym, Value>,
}

#[derive(Debug, PartialEq)]
pub struct BoundMethod {
    pub reciever: Value,
    pub method: Managed<Function>,
}
