use crate::compiler::Compiler;
use crate::parser::Parser;
use crate::vm::VM;

// Remove these
use crate::error::RuntimeError;
use crate::memory::Value;

// Bytecode
mod chunk;
mod class;
mod constant;
mod debug;
mod distance;
mod function;
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
mod gc;
mod memory;
mod vm;

fn main() {
    let buf = "
        @myclass {}

        $c = myclass();
        c.bacon = true;
        print(c.bacon);
    ";

    let mut parser = Parser::new(buf);
    let statements = match parser.parse() {
        Ok(stmts) => stmts,
        Err(e) => panic!(format!("{:?}", e)),
    };

    println!("ast: {:?}\n", statements);

    let mut compiler = Compiler::new(parser.strings);
    let mut module = match compiler.compile(&statements) {
        Ok(module) => module.clone(),
        Err(e) => panic!(format!("{:?}", e)),
    };

    let mut vm = VM::new(&mut module);

    // TODO :- Move this to own library
    vm.set_native_fn("clock", 0, |_args| {
        use std::time::{SystemTime, UNIX_EPOCH};

        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        Ok(Value::from(time))
    });

    // TODO :- Move this to own library
    vm.set_native_fn("len", 1, |_args| match &_args[0] {
        Value::Array(elements) => {
            return Ok(Value::from(elements.len() as f64));
        }
        _ => return Err(RuntimeError::ExpectedArray),
    });

    match vm.interpret() {
        Ok(_) => (),
        Err(e) => panic!(format!("{:?}", e)),
    }
}
