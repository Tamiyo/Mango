use crate::chunk::Chunk;
use crate::constant::Constant;
use crate::error::RuntimeError;
use crate::function::Closure;
use crate::function::Function;
use crate::module::Module;
use std::collections::HashMap;
use string_interner::Sym;
#[derive(Debug, PartialEq)]
enum InterpreterResult {
    More,
    Done,
}

#[derive(Debug)]
pub struct CallFrame<'a> {
    closure: Closure,
    ip: usize,
    base_counter: usize,
    chunk: &'a Chunk,
}

#[derive(Debug)]
pub struct VM<'a> {
    module: &'a Module<'a>,
    frames: Vec<CallFrame<'a>>,
    stack: Vec<Constant>,
    globals: HashMap<Sym, Constant>,
}

impl<'a> VM<'a> {
    pub fn new(module: &'a Module) -> Self {
        VM {
            module,
            frames: vec![],
            stack: vec![],
            globals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<(), RuntimeError> {
        Ok(())
    }
}
