use crate::vm::error::RuntimeError;
use crate::vm::memory::Value;
use crate::vm::natives::function::NativeFn;
use crate::vm::vm::VM;

const NATIVE_FN: [NativeFn; 1] = [NativeFn {
    name: "len",
    arity: 1,
    code: |_args| match &_args[0] {
        Value::Array(elements) => Ok(Value::Number(elements.len() as f64)),
        _ => Err(RuntimeError::ExpectedArray),
    },
}];

pub fn link_natives(vm: &mut VM) {
    for native in &NATIVE_FN {
        vm.set_native_fn(native.name, native.arity, native.code);
    }
}
