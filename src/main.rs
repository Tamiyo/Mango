use crate::compiler::Compiler;
use crate::parser::Parser;
use crate::vm::VM;

// Move these
use crate::constant::Constant;
use crate::error::RuntimeError;
use crate::memory::Distance;

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
    let buf = "
        $start = clock();
        $A = ['Hello World', 'Goodbye World', 4, 12.0];
        for j in A {
            print(j);
        }
        print(A);
        print('\nelapsed time: ' + (clock() - start));
    ";

    let mut parser = Parser::new(buf);
    let statements = match parser.parse() {
        Ok(stmts) => stmts,
        Err(e) => panic!(format!("{:?}", e)),
    };

    println!("ast: {:?}\n", statements);

    let mut compiler = Compiler::new(parser.strings);
    let module = match compiler.compile(&statements) {
        Ok(module) => module,
        Err(e) => panic!(format!("{:?}", e)),
    };

    let mut vm = VM::new(module.clone());

    // TODO :- Move this to own library
    vm.set_native_fn("clock", 0, |_args| {
        use std::time::{SystemTime, UNIX_EPOCH};

        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        Ok(Constant::from(time))
    });

    vm.set_native_fn("len", 1, |_args| match &_args[0] {
        Constant::Array(elements) => {
            return Ok(Constant::from(elements.len() as f64));
        }
        _ => return Err(RuntimeError::ExpectedArray),
    });

    // vm.set_native_fn("len", |_args| {

    // });

    match vm.interpret() {
        Ok(_) => (),
        Err(e) => panic!(format!("{:?}", e)),
    }
}
