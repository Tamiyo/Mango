use std::iter::Peekable;
use std::str::Chars;
use crate::token::{Symbol, Token};

pub struct Scanner<'a> {
    it: Peekable<Chars<'a>>,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(buf: &'a str) -> Self {
        Scanner {
            it: buf.chars().peekable(),
            line: 0,
        }
    }

    fn consume_while<F>(&mut self, x: F) -> Vec<char>
        where F: Fn(char) -> bool
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
        if self.it.peek().expect("expected peek symbol") == &expected {
            self.it.next();
            pass
        } else {
            fail
        }
    }

    fn whitespace(&mut self) {
        loop {
            match self.it.peek() {
                Some(ch) => {
                    match ch {
                        '\n' => {
                            self.line += 1;
                            self.it.next();
                        }
                        '\r' | '\t' | ' ' => { self.it.next(); }
                        _ => break
                    }
                }
                _ => break
            }
        }
    }

    fn number(&mut self, x: char) -> Symbol {
        let mut result = String::new();
        result.push(x);

        let integer: String = self.consume_while(|c| c.is_numeric())
            .into_iter()
            .collect();
        result.push_str(integer.as_str());

        if self.it.peek() == Some(&'.') {
            self.it.next();
            let decimal: String = self.consume_while(|c| c.is_numeric())
                .into_iter()
                .collect();
            result.push('.');
            result.push_str(decimal.as_str());
        }

        Symbol::Number(result.parse::<f64>().expect("expected f64 parse conversion"))
    }

    fn string(&mut self, delim: char) -> Symbol {
        let mut result: String = self.consume_while(|c| c != delim)
            .into_iter()
            .collect();
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
            "my" => Symbol::My,
            "none" => Symbol::And,
            "or" => Symbol::Or,
            "print" => Symbol::Print,
            "return" => Symbol::Return,
            "super" => Symbol::Super,
            "true" => Symbol::True,
            "while" => Symbol::While,
            _ => Symbol::Identifier(name)
        }
    }

    fn identifier(&mut self, x: char) -> Symbol {
        let mut result = String::new();
        result.push(x);

        let rest: String = self.consume_while(|c| c.is_alphanumeric() || c == '_')
            .into_iter()
            .collect();
        result.push_str(rest.as_str());

        self.keyword(result)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            self.whitespace();

            let ch = match self.it.next() {
                Some(ch) => ch,
                None => break
            };

            let result = match ch {
                '!' => self.either('=', Symbol::NotEqual, Symbol::Not),
                '=' => self.either('=', Symbol::EqualEqual, Symbol::Equal),
                '<' => self.either('=', Symbol::LessEqual, Symbol::Less),
                '>' => self.either('=', Symbol::GreaterEqual, Symbol::Greater),
                '+' => self.either('=', Symbol::PlusEqual, Symbol::Plus),
                '-' => self.either('=', Symbol::MinusEqual, Symbol::Minus),
                '*' => self.either('=', Symbol::StarEqual, Symbol::Star),
                '/' => self.either('=', Symbol::SlashEqual, Symbol::Slash),
                '%' => self.either('=', Symbol::ModuloEqual, Symbol::Modulo),
                '^' => self.either('=', Symbol::CaratEqual, Symbol::Carat),
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
                '@' => Symbol::Fun,
                '#' => Symbol::Struct,
                x if x.is_numeric() => self.number(x),
                x if x.is_alphabetic() => self.identifier(x),
                '\'' | '"' => self.string(ch),
                _ => break
            };

            tokens.push(Token::new(result, self.line));
        }
        tokens.push(Token::new(Symbol::Eof, self.line));
        tokens
    }
}