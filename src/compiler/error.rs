#[derive(Debug)]
pub enum CompileError {
    VariableNotInitialized,
    UnexpectedExpression,
    ReturnInScript,
}
