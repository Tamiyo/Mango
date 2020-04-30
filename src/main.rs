use crate::compiler::Compiler;
use crate::parser::Parser;
use crate::vm::VM;

// Bytecode
mod chunk;
mod class;
mod constant;
mod debug;
mod function;
mod memory;
mod module;

// Utility
mod error;

// Front-End
mod ast;
mod parser;
mod scanner;
mod tokens;

// Back-End
mod compiler;
mod local;

// VM
mod vm;

fn main() {
    let buf = "if (true) { print('hello world!'); } else { print('Nope!'); }";

    let mut parser = Parser::new(buf);
    let statements = match parser.parse() {
        Ok(stmts) => stmts,
        Err(e) => panic!(format!("{:?}", e)),
    };

    println!("ast: {:?}", statements);

    let mut compiler = Compiler::new(&parser.strings);
    let module = match compiler.compile(&statements) {
        Ok(module) => module,
        Err(e) => panic!(format!("{:?}", e)),
    };
}
