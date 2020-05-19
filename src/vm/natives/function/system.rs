use crate::vm::memory::Value;
use crate::vm::natives::function::NativeFn;
use crate::vm::vm::VM;

const NATIVE_FN: [NativeFn; 1] = [NativeFn {
    name: "clock",
    arity: 0,
    code: |_args| {
        use std::time::{SystemTime, UNIX_EPOCH};

        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        Ok(Value::Number(time as f64))
    },
}];

pub fn link_natives(vm: &mut VM) {
    for native in &NATIVE_FN {
        vm.set_native_fn(native.name, native.arity, native.code);
    }
}
