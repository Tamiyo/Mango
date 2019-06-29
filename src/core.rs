// Enums that the compiler references
// pub enum Keyword {
//     If,
//     Else,
//     Elif,
//     For,
//     While,
//     Define,
// }

pub enum Symbol {
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
}

pub enum Types {
    Float,
    Integer,
    String,
    Term
}

// Structs that are needed throughout the compiler
pub struct LexerResult<'a> {
    pub token: String,
    pub ttype: &'a str,
}

impl<'a> LexerResult<'a> {
    pub fn to_string(&self) -> String {
        return format!("LexerResult: (token: {}, type: {})", self.token, self.ttype);
    }
}
