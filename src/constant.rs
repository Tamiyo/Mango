use crate::memory::Distance;
use string_interner::Sym;

use crate::class::Class;
use crate::function::{Closure, Function};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Constant {
    // Primitive Constants
    Number(Distance),
    String(Sym),
    Boolean(bool),
    None,

    // Data Structure Constants
    Array(Vec<Constant>),

    // Non-Primitive Constants
    Closure(Closure),
    Class(Class),
}

impl From<f64> for Constant {
    fn from(item: f64) -> Self {
        Constant::Number(Distance::from(item))
    }
}

impl From<&Distance> for Constant {
    fn from(item: &Distance) -> Self {
        Constant::Number(item.clone())
    }
}

impl From<&Sym> for Constant {
    fn from(item: &Sym) -> Self {
        Constant::String(*item)
    }
}

impl From<&bool> for Constant {
    fn from(item: &bool) -> Self {
        Constant::Boolean(*item)
    }
}

impl From<Vec<Constant>> for Constant {
    fn from(item: Vec<Constant>) -> Self {
        Constant::Array(item)
    }
}

impl From<Function> for Constant {
    fn from(item: Function) -> Self {
        Constant::Closure(Closure::new(item))
    }
}

impl From<Class> for Constant {
    fn from(item: Class) -> Self {
        Constant::Class(item)
    }
}
