use crate::class::Class;
use crate::class::Instance;
use crate::function::Function;
use crate::function::NativeFunction;
use crate::managed::Managed;
use core::cell::RefCell;
use string_interner::Sym;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    // Primitives
    Number(f64),
    String(Sym),
    Boolean(bool),
    None,

    // Data Structures
    Array(Managed<Vec<Value>>),

    // Objects
    Function(Managed<Function>),
    NativeFunction(Managed<NativeFunction>),
    Class(Managed<RefCell<Class>>),
    Instance(Managed<RefCell<Instance>>),
}

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Function(a), Value::Function(b)) => a == b,
            (Value::NativeFunction(a), Value::NativeFunction(b)) => a == b,
            (Value::Class(a), Value::Class(b)) => a == b,
            (Value::Instance(a), Value::Instance(b)) => a == b,
            _ => false,
        }
    }
}
