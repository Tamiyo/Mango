/// Error Enum to define possible problems during runtime execution.
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
    ExpectedClosure,
    ExpectedClass,
    UndefinedProperty,
    UnexpectedValue,
    UnexpectedConstant,
}
