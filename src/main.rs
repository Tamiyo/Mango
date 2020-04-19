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

fn main() {
    let buf = "print(true or false);";
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
