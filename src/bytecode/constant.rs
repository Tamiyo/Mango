use crate::bytecode::distance::Distance;
use string_interner::Sym;

use crate::compiler::class::Class;
use crate::compiler::function::Closure;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Constant {
    Number(Distance),
    String(Sym),
    Closure(Closure),
    Class(Class),
}
