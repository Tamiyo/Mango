/// Defines the Scanner
///
/// The Scanner takes a user's source code as a &str and converts it into
/// a vector of Tokens.
///
/// The vector of tokens then gets sent to the parser to parse.
use crate::bytecode::distance::Distance;
use crate::parser::tokens::Symbol;
use crate::parser::tokens::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    it: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(buf: &'a str) -> Self {
        Scanner {
            it: buf.chars().peekable(),
            line: 0,
            column: 0,
        }
    }

    fn consume_while<F>(&mut self, x: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        let mut chars: Vec<char> = Vec::new();
        while let Some(&ch) = self.it.peek() {
            if x(ch) {
                self.it.next();
                chars.push(ch);
            } else {
                break;
            }
        }
        chars
    }

    fn either(&mut self, expected: char, pass: Symbol, fail: Symbol) -> Symbol {
        match self.it.peek() {
            Some(symbol) => {
                if symbol == &expected {
                    self.it.next();
                    pass
                } else {
                    fail
                }
            }
            None => fail,
        }
    }

    fn whitespace(&mut self) -> Option<Symbol> {
        while let Some(ch) = self.it.peek() {
            match ch {
                '\n' | '\r' | '\t' | ' ' => {
                    self.it.next();
                }
                '/' => {
                    self.it.next();
                    if *self.it.peek().unwrap() == '/' {
                        while let Some(ch) = self.it.peek() {
                            match ch {
                                '\n' => break,
                                _ => self.it.next(),
                            };
                        }
                    } else {
                        return Some(Symbol::Slash);
                    }
                }
                _ => break,
            }
        }

        None
    }

    fn number(&mut self, x: char) -> Symbol {
        let mut result = String::new();
        result.push(x);

        let integer: String = self.consume_while(|c| c.is_numeric()).into_iter().collect();
        result.push_str(integer.as_str());

        if self.it.peek() == Some(&'.') {
            self.it.next();
            let decimal: String = self.consume_while(|c| c.is_numeric()).into_iter().collect();
            result.push('.');
            result.push_str(decimal.as_str());
        }

        let res_f = result
            .parse::<f64>()
            .expect("expected f64 parse conversion");
        Symbol::Number(Distance::from(res_f))
    }

    fn string(&mut self, delim: char) -> Symbol {
        let result: String = self.consume_while(|c| c != delim).into_iter().collect();
        if self.it.next().expect("expected next symbol") != delim {
            panic!("Unterminated String!");
        }

        Symbol::String(result)
    }

    fn keyword(&mut self, name: String) -> Symbol {
        match name.as_str() {
            "and" => Symbol::And,
            "elif" => Symbol::Elif,
            "else" => Symbol::Else,
            "false" => Symbol::False,
            "for" => Symbol::For,
            "if" => Symbol::If,
            "in" => Symbol::In,
            "my" => Symbol::My,
            "none" => Symbol::None,
            "or" => Symbol::Or,
            "print" => Symbol::Print,
            "return" => Symbol::Return,
            "super" => Symbol::Super,
            "true" => Symbol::True,
            "while" => Symbol::While,
            _ => Symbol::Identifier(name),
        }
    }

    fn identifier(&mut self, x: char) -> Symbol {
        let mut result = String::new();
        result.push(x);

        let rest: String = self
            .consume_while(|c| c.is_alphanumeric() || c == '_')
            .into_iter()
            .collect();
        result.push_str(rest.as_str());

        self.keyword(result)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            if let Some(symbol) = self.whitespace() {
                tokens.push(Token::new(symbol, self.line, self.column));
                self.whitespace();
            };

            let ch = match self.it.next() {
                Some(ch) => {
                    if ch == '\n' {
                        self.line += 1;
                        self.column = 0;
                    } else {
                        self.column += 1;
                    }
                    ch
                }
                None => break,
            };

            let result = match ch {
                '!' => self.either('=', Symbol::NotEqual, Symbol::Not),
                '=' => self.either('=', Symbol::EqualEqual, Symbol::Equal),
                '<' => self.either('=', Symbol::LessEqual, Symbol::Less),
                '>' => self.either('=', Symbol::GreaterEqual, Symbol::Greater),
                '+' => Symbol::Plus,
                '-' => Symbol::Minus,
                '*' => Symbol::Star,
                '/' => Symbol::Slash,
                '%' => Symbol::Modulo,
                '^' => Symbol::Carat,
                '(' => Symbol::LeftParen,
                ')' => Symbol::RightParen,
                '{' => Symbol::LeftBrace,
                '}' => Symbol::RightBrace,
                '[' => Symbol::LeftSquare,
                ']' => Symbol::RightSquare,
                ',' => Symbol::Comma,
                '.' => Symbol::Dot,
                ':' => Symbol::Colon,
                ';' => Symbol::Semicolon,
                '#' => Symbol::Fun,
                '@' => Symbol::Class,
                '$' => Symbol::Var,
                x if x.is_numeric() => self.number(x),
                x if x.is_alphabetic() => self.identifier(x),
                '\'' | '"' => self.string(ch),
                _ => {
                    println!("Broke on: {:?}", ch);
                    break;
                }
            };

            tokens.push(Token::new(result, self.line, self.column));
        }
        tokens.push(Token::new(Symbol::Eof, self.line, self.column));
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::bytecode::distance::Distance;
    use crate::parser::tokens::Symbol;

    fn tokenize(buf: &str) -> Vec<Symbol> {
        let mut scanner = Scanner::new(buf);
        scanner
            .tokenize()
            .iter()
            .map(|token| token.symbol.clone())
            .collect()
    }

    #[test]
    fn test() {
        assert_eq!(tokenize(""), [Symbol::Eof]);
        assert_eq!(tokenize("("), [Symbol::LeftParen, Symbol::Eof]);
        assert_eq!(tokenize(")"), [Symbol::RightParen, Symbol::Eof]);
        assert_eq!(tokenize("{"), [Symbol::LeftBrace, Symbol::Eof]);
        assert_eq!(tokenize("}"), [Symbol::RightBrace, Symbol::Eof]);
        assert_eq!(tokenize("["), [Symbol::LeftSquare, Symbol::Eof]);
        assert_eq!(tokenize("]"), [Symbol::RightSquare, Symbol::Eof]);
        assert_eq!(tokenize(","), [Symbol::Comma, Symbol::Eof]);
        assert_eq!(tokenize(":"), [Symbol::Colon, Symbol::Eof]);
        assert_eq!(tokenize("."), [Symbol::Dot, Symbol::Eof]);
        assert_eq!(tokenize("-"), [Symbol::Minus, Symbol::Eof]);
        assert_eq!(tokenize("+"), [Symbol::Plus, Symbol::Eof]);
        assert_eq!(tokenize(";"), [Symbol::Semicolon, Symbol::Eof]);
        assert_eq!(tokenize("/"), [Symbol::Slash, Symbol::Eof]);
        assert_eq!(tokenize("%"), [Symbol::Modulo, Symbol::Eof]);
        assert_eq!(tokenize("^"), [Symbol::Carat, Symbol::Eof]);
        assert_eq!(tokenize("*"), [Symbol::Star, Symbol::Eof]);
        assert_eq!(tokenize("!"), [Symbol::Not, Symbol::Eof]);
        assert_eq!(tokenize("!="), [Symbol::NotEqual, Symbol::Eof]);
        assert_eq!(tokenize("="), [Symbol::Equal, Symbol::Eof]);
        assert_eq!(tokenize("=="), [Symbol::EqualEqual, Symbol::Eof]);
        assert_eq!(tokenize(">"), [Symbol::Greater, Symbol::Eof]);
        assert_eq!(tokenize(">="), [Symbol::GreaterEqual, Symbol::Eof]);
        assert_eq!(tokenize("<"), [Symbol::Less, Symbol::Eof]);
        assert_eq!(tokenize("<="), [Symbol::LessEqual, Symbol::Eof]);
        assert_eq!(
            tokenize("ident_4"),
            [Symbol::Identifier("ident_4".to_string()), Symbol::Eof]
        );
        assert_eq!(
            tokenize("\"4 -str\""),
            [Symbol::String("4 -str".to_string()), Symbol::Eof]
        );
        assert_eq!(
            tokenize("\'4 -str\'"),
            [Symbol::String("4 -str".to_string()), Symbol::Eof]
        );
        assert_eq!(
            tokenize("4"),
            [Symbol::Number(Distance::from(4.0)), Symbol::Eof]
        );
        assert_eq!(
            tokenize("4.5"),
            [Symbol::Number(Distance::from(4.5)), Symbol::Eof]
        );
        assert_eq!(tokenize("and"), [Symbol::And, Symbol::Eof]);
        assert_eq!(tokenize("#"), [Symbol::Fun, Symbol::Eof]);
        assert_eq!(tokenize("@"), [Symbol::Class, Symbol::Eof]);
        assert_eq!(tokenize("elif"), [Symbol::Elif, Symbol::Eof]);
        assert_eq!(tokenize("else"), [Symbol::Else, Symbol::Eof]);
        assert_eq!(tokenize("false"), [Symbol::False, Symbol::Eof]);
        assert_eq!(tokenize("for"), [Symbol::For, Symbol::Eof]);
        assert_eq!(tokenize("if"), [Symbol::If, Symbol::Eof]);
        assert_eq!(tokenize("my"), [Symbol::My, Symbol::Eof]);
        assert_eq!(tokenize("none"), [Symbol::None, Symbol::Eof]);
        assert_eq!(tokenize("or"), [Symbol::Or, Symbol::Eof]);
        assert_eq!(tokenize("print"), [Symbol::Print, Symbol::Eof]);
        assert_eq!(tokenize("return"), [Symbol::Return, Symbol::Eof]);
        assert_eq!(tokenize("super"), [Symbol::Super, Symbol::Eof]);
        assert_eq!(tokenize("true"), [Symbol::True, Symbol::Eof]);
        assert_eq!(tokenize("while"), [Symbol::While, Symbol::Eof]);
        assert_eq!(tokenize("$"), [Symbol::Var, Symbol::Eof]);
    }
}
