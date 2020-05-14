use crate::compiler::Compiler;
use crate::module::Module;
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
mod managed;
mod memory;
mod vm;

// Try to implement this "managed" code
// https://github.com/Manishearth/rust-gc/blob/master/gc/src/lib.rs
// https://github.com/Manishearth/rust-gc/blob/master/gc/src/gc.rs
fn main() {
    // let buf = "
    //     @myclass {
    //         #eggs() {
    //             print('Hello... eggs?');
    //         }

    //         #omlet(x) {
    //             print('What a nice omlet, made with eggs and', x);
    //         }
    //     }

    //     $c = myclass();
    //     c.bacon = true;
    //     print('c.bacon:', c.bacon);
    //     c.eggs();
    //     c.omlet('potatoes');
    // ";

    let buf = "
        $A = [1, 2, 3];
        $B = [4, 5, 6];

        print(A);
        print(B);
        for i in B {
            print(i);
        }
    ";

    let mut parser = Parser::new(buf);
    let statements = match parser.parse() {
        Ok(stmts) => stmts,
        Err(e) => panic!(format!("{:?}", e)),
    };

    println!("ast: {:?}\n", statements);

    let mut compiler = Compiler::new(parser.strings);
    let mut module: Module = match compiler.compile(&statements) {
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
        Ok(Value::Number(time))
    });

    // TODO :- Move this to own library
    vm.set_native_fn("len", 1, |_args| match &_args[0] {
        Value::Array(elements) => {
            return Ok(Value::Number(elements.len() as f64));
        }
        _ => return Err(RuntimeError::ExpectedArray),
    });

    match vm.interpret() {
        Ok(_) => (),
        Err(e) => panic!(format!("{:?}", e)),
    }
}
