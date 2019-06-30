use crate::lexer::Lexer;
use crate::parser::Parser;
use std::collections::HashMap;

mod lexer;
mod parser;
mod core;

fn main() {
    let input_string = "myident = 12.0\n";
    let lexer = Lexer { input: input_string };
    let stack = lexer.lex();

    println!("stack: {:?}", stack);

    let mut parser = Parser {
        stack: vec![],
        token_stack:  stack,
        node_stack: vec![],
        action: HashMap::new(),
        goto: HashMap::new()
    };

    parser.parse();
}
