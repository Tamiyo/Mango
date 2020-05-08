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
    // let buf = "
    // #PrintFunction() {
    //     $y = 5;

    //     #InnerFunction() {
    //         print(y + 1);
    //         return y + 2;
    //     }

    //     print('Hello World');
    //     $x = 4;
    //     print(3 + x);
    //     return x;
    // }";

    let buf = "
        $A = [[[1, 2], [3, 4]], [4, 5, 6], [7, 8, 9]];
        print(A[0]);
        print(A[0][0]);
        print(A[1:]);
        print(A[2][1:]);
    ";

    // Problem is in the parser for nested []. Maybe include a vec?
    // Maybe a grammar problem...

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
    match vm.interpret() {
        Ok(_) => (),
        Err(e) => panic!(format!("{:?}", e)),
    }
}
