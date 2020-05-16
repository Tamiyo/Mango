#[derive(Debug)]
pub enum RuntimeError {
    StackEmpty,
    CallFrameEmpty,
    IndexOutOfBounds,
    IncorrectArity,
    UndefinedVariable,
    ExpectedStringConstant,
    ExpectedArray,
    ExpectedNumber,
    ExpectedCallee,
    ExpectedInstance,
    ExpectedClass,
    UndefinedProperty,
    UnexpectedValue,
    UnexpectedConstant,
}
