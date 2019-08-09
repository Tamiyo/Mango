use std::collections::HashMap;

use crate::core::{LexerResult, Scope};
use crate::lexer::Lexer;
use crate::parse_tree::Node;
use crate::parser::Parser;

#[macro_use]
extern crate lazy_static;

mod core;
mod lexer;
mod parser;
mod parse_tree;
mod semantic_analyzer;


static SCOPE_LEVEL: i32 = 1;
static SCOPE_LIMIT: i32 = 256;

lazy_static! {
    static ref SCOPED_SYMBOL_TABLE: HashMap<i32, Scope> = HashMap::new();
}

fn main() {
    #![allow(dead_code)]
    let input_string = "if 3 <= 4 { 3 + 4 }$";
    let lexer = Lexer { input: input_string };
    let stack = lexer.lex();

    let mut parser = Parser {
        token_stack: stack,
        action: HashMap::new(),
        goto: HashMap::new(),
    };

    let top_node = parser.parse();
    top_node.eval(SCOPE_LEVEL);
}
