#[derive(Debug)]
pub enum CompileError {
    VariableNotInitialized,
    UnexpectedExpression,
    MyUsedOutsideClass,
    ReturnInScript,
    ReturnInInitializer,
}
