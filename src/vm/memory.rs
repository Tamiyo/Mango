/// The runtime implementation of possible Values that present themselves
/// during execution.
///
/// The Values here are given more state than their compile-time constants
/// brethern, and thus a redefinition of them here is needed to keep track
/// of the new state.
use crate::bytecode::chunk::StackIndex;
use crate::vm::class::BoundMethod;
use crate::vm::class::Class;
use crate::vm::class::Instance;
use crate::vm::function::Closure;
use crate::vm::function::NativeFunction;
use crate::vm::gc::managed::Gc;
use crate::vm::gc::managed::Trace;
use core::cell::RefCell;
use string_interner::Sym;
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Upvalue {
    Open(StackIndex),
    Closed(Value),
}

impl Upvalue {
    pub fn is_open(&self, index: StackIndex) -> bool {
        match self {
            Upvalue::Open(i) => *i == index,
            Upvalue::Closed(_) => false,
        }
    }

    pub fn open(&self) -> bool {
        match self {
            Upvalue::Open(_) => true,
            Upvalue::Closed(_) => false,
        }
    }
}

impl Trace for Upvalue {
    fn trace(&self) {
        match self {
            Upvalue::Open(_) => (),
            Upvalue::Closed(value) => value.trace(),
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    // Primitives
    Number(f64),
    String(Sym),
    Boolean(bool),
    None,

    // Data Structures
    Array(Gc<Vec<Value>>),

    // Objects
    Closure(Gc<Closure>),
    NativeFunction(Gc<NativeFunction>),

    Class(Gc<RefCell<Class>>),
    Instance(Gc<RefCell<Instance>>),

    BoundMethod(Gc<BoundMethod>),
}

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => (**a) == (**b),
            (Value::Closure(a), Value::Closure(b)) => (**a) == (**b),
            (Value::NativeFunction(a), Value::NativeFunction(b)) => (**a) == (**b),
            (Value::Class(a), Value::Class(b)) => (**a) == (**b),
            (Value::Instance(a), Value::Instance(b)) => (**a) == (**b),
            _ => false,
        }
    }
}

impl Trace for Sym {
    fn trace(&self) {}
}

impl Trace for Value {
    fn trace(&self) {
        match self {
            Value::String(string) => string.trace(),
            Value::NativeFunction(function) => function.trace(),
            Value::Closure(closure) => closure.trace(),
            Value::Class(class) => class.trace(),
            Value::Array(elements) => elements.trace(),
            Value::Instance(instance) => instance.trace(),
            Value::BoundMethod(method) => method.trace(),
            Value::Number(_) => (),
            Value::None => (),
            Value::Boolean(_) => (),
        }
    }
}
