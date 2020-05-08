use crate::tokens::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedOperator(Token),
    UnexpectedToken(Token),
    ExpectedIdentifier(Token),
    ExpectedLiteral(Token),
    TokenStreamEmpty,
}
#[derive(Debug)]
pub enum CompileError {
    UndefinedVariable,
<<<<<<< HEAD
    ContextStreamEmpty,
=======
>>>>>>> 382353fd91b0585622e95c4ebfd4e877abef4353
    UnexpectedBinaryOperator(Token),
    UnexpectedLogicalOperator(Token),
    UnexpectedUnaryOperator(Token),
    ReturnInScript,
}
#[derive(Debug)]
<<<<<<< HEAD
pub enum RuntimeError {
    StackEmpty,
    CallFrameEmpty,
    IndexOutOfBounds,
    IncorrectArity,
    UndefinedVariable,
    ExpectedStringConstant,
    ExpectedArray,
    ExpectedNumber,
    ExpectedCallee
}
=======
pub enum RuntimeError {}
>>>>>>> 382353fd91b0585622e95c4ebfd4e877abef4353
