use crate::bytecode::module::Module;
use crate::compiler::compiler::Compiler;
use crate::parser::parser::Parser;
use crate::vm::vm::VM;

mod bytecode;
mod compiler;
mod parser;
mod vm;

fn main() {
    let buf = "
       @Oops {
           #init() {
                #f() {
                    #g() {
                        #j() {
                            my.r = 4;
                        }
                        j();
                    }
                    print('Not a method');
                    g();
                }
                my.field = f;
            }
       }

       oops = Oops();
       oops.field();
       print(oops.r);
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
    // vm.set_native_fn("clock", 0, |_args| {
    //     use std::time::{SystemTime, UNIX_EPOCH};

    //     let time = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap_or_default()
    //         .as_secs_f64();
    //     Ok(Value::Number(time))
    // });

    // TODO :- Move this to own library
    // vm.set_native_fn("len", 1, |_args| match &_args[0] {
    //     Value::Array(elements) => {
    //         Ok(Value::Number(elements.len() as f64))
    //     }
    //     _ => Err(RuntimeError::ExpectedArray),
    // });

    match vm.interpret() {
        Ok(_) => (),
        Err(e) => panic!(format!("{:?}", e)),
    }
}
