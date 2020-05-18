/// Error Enum to define possible problems during parsing.
use crate::parser::tokens::Token;

#[derive(Debug)]
pub enum ParseError {
    ExpectedAssignableValue,
    UnexpectedOperator(Token),
    UnexpectedToken(Token),
    ExpectedIdentifier(Token),
    ExpectedLiteral(Token),
    TokenStreamEmpty,
}
