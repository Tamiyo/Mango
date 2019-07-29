use std::collections::HashMap;

use crate::lexer::Lexer;
use crate::parser::Parser;

mod lexer;
mod parser;
mod core;
mod parse_tree;

fn main() {
    #![allow(dead_code)]
    let input_string = "12 + 12\n$";
    let lexer = Lexer { input: input_string };
    let stack = lexer.lex();

    let mut parser = Parser {
        token_stack: stack,
        action: HashMap::new(),
        goto: HashMap::new(),
    };

    let top_node = parser.parse();
    println!("{}", top_node.eval());
}
