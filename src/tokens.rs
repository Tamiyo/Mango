use crate::distance::Distance;

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    // Single-character Symbols.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquare,
    RightSquare,
    Comma,
    Colon,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Modulo,
    Carat,
    Star,
    // One or two character Symbols.
    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier(String),
    String(String),
    Number(Distance),
    // Keywords.
    And,
    Class,
    Elif,
    Else,
    False,
    Fun,
    For,
    If,
    In,
    My,
    None,
    Or,
    Print,
    Return,
    Super,
    True,
    While,
    Var,
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub symbol: Symbol,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(symbol: Symbol, line: usize, col: usize) -> Self {
        Token { symbol, line, col }
    }
}
