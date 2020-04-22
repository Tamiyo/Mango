use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vm::VM;
use crate::module::Module;

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

// vm
mod vm;
mod function;

// general
mod error;

fn main() {
    // let buf = "#myfunction() { print(5); return; } myfunction();";
    // let buf = "$x = 0; if (x < 4) { print(2^2 + 3^2); }";
    let buf = "#pythagorean(a, b) { return (a^2 + b^2) ^ (0.5); } $c = pythagorean(1, 2); print(c);";
    let mut parser = Parser::new(buf);
    let stmts = match parser.parse() {
        Ok(s) => s,
        Err(e) => panic!(e.error)
    };

    println!("{:?}\n", stmts);

    let mut compiler = Compiler::new();

    let empty_module = Module::new();
    let module = compiler.compile(&stmts).unwrap_or(&empty_module);

    let mut vm = VM::new(module);
    match vm.interpret() {
        Ok(t) => println!("Interpret complete!"),
        Err(e) => panic!(e.error)
    }
}
