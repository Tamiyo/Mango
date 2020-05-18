/// Error Enum to define possible problems during compilation.

#[derive(Debug)]
pub enum CompileError {
    VariableNotInitialized,
    UnexpectedExpression,
    MyUsedOutsideClass,
    ReturnInScript,
    ReturnInInitializer,
}
