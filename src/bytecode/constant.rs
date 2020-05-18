/// Defines the 'Constants' to be used within the Compiler.
/// 
/// These are constants that the Compiler comes by during it's compilation phase,
/// and they get stored in the Constant Pool for later use.
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
