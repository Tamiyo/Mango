use crate::vm::error::RuntimeError;
use crate::vm::memory::Value;
use crate::vm::vm::VM;

mod system;
mod util;

struct NativeFn<'a> {
    name: &'a str,
    arity: usize,
    code: fn(&[Value]) -> Result<Value, RuntimeError>,
}

pub fn link_natives(vm: &mut VM) {
    system::link_natives(vm);
    util::link_natives(vm);
}
