use crate::vm::vm::VM;

mod class;
mod function;

pub fn link_natives(vm: &mut VM) {
    function::link_natives(vm);
    class::link_natives(vm);
}
