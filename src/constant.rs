use crate::distance::Distance;
use string_interner::Sym;

use crate::class::ConstantClass;
use crate::function::Function;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Constant {
    // Primitive Constants
    Number(Distance),
    String(Sym),
    Function(Function),
    Class(ConstantClass),
}

impl From<f64> for Constant {
    fn from(item: f64) -> Self {
        Constant::Number(Distance::from(item))
    }
}

impl From<&Sym> for Constant {
    fn from(item: &Sym) -> Self {
        Constant::String(*item)
    }
}

impl From<Function> for Constant {
    fn from(item: Function) -> Self {
        Constant::Function(item)
    }
}

impl From<&Function> for Constant {
    fn from(item: &Function) -> Self {
        Constant::Function(item.clone())
    }
}

impl From<ConstantClass> for Constant {
    fn from(item: ConstantClass) -> Self {
        Constant::Class(item)
    }
}
