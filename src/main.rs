use crate::parser::Parser;
use crate::compiler::Compiler;

// bytecode
mod chunk;
mod constant;
mod debug;

// parser
mod token;
mod ast;
mod scanner;
mod parser;

// compiler
mod compiler;
mod module;
mod local;

// general
mod error;

#[derive(Debug)]
struct A {
    pub left: B,
    pub right: B,
}

#[derive(Debug)]
struct B {
    pub val: i32,
}

fn main() {
    let buf = "if (true) { print('if'); } elif (false) { print('elif'); } else { print('else'); }";
    let mut parser = Parser::new(buf);
    let stmts = match parser.parse() {
        Ok(s) => s,
        Err(e) => panic!(e.error)
    };

    println!("{:?}\n", stmts);

    let mut compiler = Compiler::new();
    compiler.program(&stmts);
    compiler.disassemble();
}
