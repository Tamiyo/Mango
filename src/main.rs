use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::parser::Parser;
#[macro_use]
extern crate downcast_rs;

mod core;
mod lexer;
mod parser;
mod parse_tree;
mod semantic_analyzer;

fn main() {
    #![allow(dead_code)]
    let input_string = "13 + 12\n$";
    let lexer = Lexer { input: input_string };
    let stack = lexer.lex();

    let mut parser = Parser {
        token_stack: stack,
        action: HashMap::new(),
        goto: HashMap::new(),
    };

    let top_node = parser.parse();
    println!("evaluation: {}", top_node.eval());
}
