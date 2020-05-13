use crate::memory::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use string_interner::Sym;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ConstantClass {
    pub name: Sym,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VMClass {
    pub name: Sym,
    pub methods: HashMap<Sym, Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instance {
    pub class: RefCell<VMClass>,
    pub fields: HashMap<Sym, Value>,
}
