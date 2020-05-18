/// Defines a compile-time implementation of 'Class' to use during compilation
use string_interner::Sym;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Class {
    pub name: Sym,
}
