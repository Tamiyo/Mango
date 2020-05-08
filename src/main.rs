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
<<<<<<< HEAD
    let buf = "";
=======
    let buf = "if (true) { print('hello world!'); } else { print('Nope!'); }";
>>>>>>> 382353fd91b0585622e95c4ebfd4e877abef4353

    let mut parser = Parser::new(buf);
    let statements = match parser.parse() {
        Ok(stmts) => stmts,
        Err(e) => panic!(format!("{:?}", e)),
    };

<<<<<<< HEAD
    println!("ast: {:?}\n", statements);

    let mut compiler = Compiler::new(parser.strings);
=======
    println!("ast: {:?}", statements);

    let mut compiler = Compiler::new(&parser.strings);
>>>>>>> 382353fd91b0585622e95c4ebfd4e877abef4353
    let module = match compiler.compile(&statements) {
        Ok(module) => module,
        Err(e) => panic!(format!("{:?}", e)),
    };
<<<<<<< HEAD

    let mut vm = VM::new(module.clone());
    match vm.interpret() {
        Ok(_) => (),
        Err(e) => panic!(format!("{:?}", e)),
    }
=======
>>>>>>> 382353fd91b0585622e95c4ebfd4e877abef4353
}
