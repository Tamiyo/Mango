use crate::parser::Parser;

// Enums that the compiler references
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum TokenType {
    Term,
    Identifier,

    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,

    Equals,
    DoubleEquals,
    TripleEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Not,

    LeftCurlyBrace,
    RightCurlyBrace,
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Colon,
    Semicolon,
    Newline,
    EndOfFile,

    For,
    While,
    Define,
    If,
    Elif,
    Else,

    None,

    Mango,
    StatementSuite,
    StatementList,
    Statement,
    StatementSimple,
    StatementComplex,
    StatementExpression,
    StatementExpressionP,
    StatementExpression2,
    StatementExpression2p,
    StatementExpression3,
    StatementAssignment,
    StatementConditional,
    StatementConditionalElif,
    StatementConditionalElse,
    StatementConditionalTest,
    OperatorBinary,
    OperatorUnary,
    StatementLoop,
    StatementLoopFor,
    StatementLoopForOptions,
    StatementLoopWhile,
    StatementDefineFunction,
    FunctionParams,
    StatementDefineClass,
}

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Float,
    Integer,
    String,
    Boolean,
    Object,
    None,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ParserAction {
    Shift,
    Reduce,
    Accept,
    Goto,
}

// Structs that are needed throughout the compiler
#[derive(Debug, Clone)]
pub struct LexerResult {
    pub token: String,
    pub inferred_type: PrimitiveType,
    pub token_type: TokenType,
}

impl LexerResult {
    pub fn to_string(&self) -> String {
        return format!("LexerResult: (token_type: {:?})", self.token_type);
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ActionNode {
    pub action: ParserAction,
    pub value: i32,
}

impl ActionNode {
    pub fn to_string(&self) -> String {
        return format!("ActionNode: (action: {:?}, value: {:?})", self.action, self.value);
    }
}


#[derive(Debug, Eq, PartialEq, Hash)]
pub struct GotoNode {
    pub token_type: TokenType,
    pub value: i32,
}

pub fn symbol_to_enum(symbol: &str) -> TokenType {
    match symbol {
        "+" => TokenType::Add,
        "-" => TokenType::Subtract,
        "*" => TokenType::Multiply,
        "/" => TokenType::Divide,
        "%" => TokenType::Modulo,
        "^" => TokenType::Exponent,
        "=" => TokenType::Equals,
        "==" => TokenType::DoubleEquals,
        "===" => TokenType::TripleEquals,
        "<" => TokenType::LessThan,
        "<=" => TokenType::LessThanEquals,
        ">" => TokenType::GreaterThan,
        ">=" => TokenType::GreaterThanEquals,
        "!" => TokenType::Not,
        "{" => TokenType::LeftCurlyBrace,
        "}" => TokenType::RightCurlyBrace,
        "(" => TokenType::LeftParenthesis,
        ")" => TokenType::RightParenthesis,
        "," => TokenType::Comma,
        ":" => TokenType::Colon,
        ";" => TokenType::Semicolon,
        "\n" => TokenType::Newline,
        "$" => TokenType::EndOfFile,
        _ => TokenType::None,
    }
}