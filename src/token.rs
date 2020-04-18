#[derive(PartialEq, Debug, Clone)]
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
    MinusEqual,
    Plus,
    PlusEqual,
    Semicolon,
    Slash,
    SlashEqual,
    Modulo,
    ModuloEqual,
    Carat,
    CaratEqual,
    Star,
    StarEqual,
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
    Number(f64),
    // Keywords.
    And,
    Struct,
    Elif,
    Else,
    False,
    Fun,
    For,
    If,
    My,
    None,
    Or,
    Print,
    Return,
    Super,
    True,
    While,
    Eof,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub value: Symbol,
    pub line: usize,
}

impl Token {
    pub fn new(value: Symbol, line: usize) -> Self {
        Token {
            value,
            line,
        }
    }
}