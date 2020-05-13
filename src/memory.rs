use crate::class::Instance;
use crate::class::VMClass;
use crate::function::Function;
use crate::function::NativeFunction;
use core::cell::RefCell;
use string_interner::Sym;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(Sym),
    Boolean(bool),
    None,

    // Data Structure Constants
    Array(Vec<Value>),

    // Non-Primitive Constants
    Function(Function),
    NativeFunction(NativeFunction),
    Class(RefCell<VMClass>),
    Instance(RefCell<Instance>),
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

impl From<f64> for Value {
    fn from(item: f64) -> Self {
        Value::Number(item)
    }
}

impl From<Sym> for Value {
    fn from(item: Sym) -> Self {
        Value::String(item)
    }
}

impl From<bool> for Value {
    fn from(item: bool) -> Self {
        Value::Boolean(item)
    }
}

impl From<Vec<Value>> for Value {
    fn from(item: Vec<Value>) -> Self {
        Value::Array(item)
    }
}
