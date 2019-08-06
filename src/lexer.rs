/*
    These are the tokens that our lexer shall recognize:

    identifier:     [a-zA-Z][a-zA-Z0-9_]*
    string:         ".+"
    integer:        [0-9]+
    float:          [0-9]+.[0-9]+
*/

use std::iter::Peekable;
use std::str::Chars;

use regex::Regex;

use crate::core::{LexerResult, PrimitiveType, symbol_to_enum, TokenType};

pub struct Lexer<'a> {
    pub input: &'a str,
}

impl<'a> Default for Lexer<'a> {
    fn default() -> Lexer<'a> {
        Lexer { input: "" }
    }
}

impl<'a> Lexer<'a> {
    pub fn lex(&self) -> Vec<LexerResult> {
        let mut tokens: Vec<LexerResult> = Vec::new();

        let mut it = self.input.chars().peekable();
        while let Some(&c) = it.peek() {
            match c {
                '0'...'9' => {
                    Self::get_numeric(&mut tokens, &mut it);
                }
                'a'...'z' | 'A'...'Z' => {
                    Self::get_identifier(&mut tokens, &mut it);
                }
                '"' => {
                    Self::get_string(&mut tokens, &mut it);
                }
                ' ' | '\t' => {
                    it.next();
                }
                _ => {
                    Self::get_symbol(&mut tokens, &mut it);
                }
            }
        }

        tokens
    }

    fn get_numeric(tokens: &mut Vec<LexerResult>, it: &mut Peekable<Chars>) {
        let mut token = String::new();
        let mut has_decimal = false;

        while let Some(&c) = it.peek() {
            match c {
                '0'...'9' => {
                    token.push(c);
                    it.next();
                }
                '.' => {
                    if !has_decimal {
                        has_decimal = true;
                        token.push(c);
                        it.next();
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }
        if has_decimal {
            let result = LexerResult {
                token,
                inferred_type: PrimitiveType::Float,
                token_type: TokenType::Term,
            };
            println!("{}", result.to_string());
            tokens.push(result);
        } else {
            let result = LexerResult {
                token,
                inferred_type: PrimitiveType::Integer,
                token_type: TokenType::Term,
            };
            println!("{}", result.to_string());
            tokens.push(result);
        }
    }

    fn get_identifier(tokens: &mut Vec<LexerResult>, it: &mut Peekable<Chars>) {
        let mut token = String::new();
        let mut past_first = false;

        while let Some(&c) = it.peek() {
            match c {
                'a'...'z' | 'A'...'Z' => {
                    past_first = true;
                    token.push(c);
                    it.next();
                }
                '0'...'9' | '_' => {
                    if past_first {
                        token.push(c);
                        it.next();
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }
        let result = LexerResult {
            token,
            inferred_type: PrimitiveType::None,
            token_type: TokenType::Identifier,
        };
        println!("{}", result.to_string());
        tokens.push(result);
    }

    fn get_string(tokens: &mut Vec<LexerResult>, it: &mut Peekable<Chars>) {
        let mut token = String::new();
        let mut inside_quotes = false;

        while let Some(&c) = it.peek() {
            match c {
                '"' => {
                    if !inside_quotes {
                        inside_quotes = true;
                        it.next();
                    } else {
                        it.next();
                        break;
                    }
                }
                _ => {
                    token.push(c);
                    it.next();
                }
            }
        }
        let result = LexerResult {
            token,
            inferred_type: PrimitiveType::String,
            token_type: TokenType::Term,
        };
        println!("{}", result.to_string());
        tokens.push(result);
    }

    fn get_symbol(tokens: &mut Vec<LexerResult>, it: &mut Peekable<Chars>) {
        let mut token = String::new();


        let mut length = 0;
        let mut previous = false;

        while let Some(&c) = it.peek() {
//            match c {
//                '+' | '-' | '*' | '/' | '%' | '^' | '!' | '{' | '}' | '(' | ')' | ',' | ':'
//                | ';' | '\n' | '$' => {
//                    if length == 0 {
//                        token.push(c);
//                        it.next();
//                    }
//                    break;
//                }
//                '=' => {
//                    if previous {
//                        token.push(c);
//                        it.next();
//                        break;
//                    } else if length < 3 {
//                        token.push(c);
//                        it.next();
//                        length += 1;
//                    } else {
//                        break;
//                    }
//                }
//                '>' | '<' => {
//                    if length < 1 {
//                        token.push(c);
//                        previous = true;
//                        it.next();
//                        length += 1;
//                    } else {
//                        break;
//                    }
//                }
//                _ => {
//                    it.next();
//                    break;
//                }
//            }
            match c {
                '+' | '-' | '*' | '/' | '%' | '^' | '!' | '{' | '}' | '(' | ')' | ',' | ':'
                | ';' | '\n' | '$' | '=' | '>' | '<' | '.' => {
                    let mut temp = token.clone();
                    temp.push(c);
                    if symbol_to_enum(temp.as_str()) == TokenType::None {
                        break;
                    } else {
                        token.push(c);
                    }
                    it.next();
                }
                _ => {
                    it.next();
                    break;
                }
            }
        }
        let token_type = symbol_to_enum(token.as_str());
        let result = LexerResult {
            token,
            inferred_type: PrimitiveType::None,
            token_type: token_type,
        };
        println!("{}", result.to_string());
        tokens.push(result);
    }
}
