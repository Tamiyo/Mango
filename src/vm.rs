use crate::function::Function;
use crate::constant::Constant;

struct CallFrame {
    function: Function,
    ip: usize,
    slots: Vec<Constant>,
}