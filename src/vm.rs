use crate::function::Function;
use crate::constant::Constant;
use crate::module::Module;
use std::collections::HashMap;
use crate::error::VMError;
use crate::chunk::{Chunk, Instruction};
use std::ops::Add;

#[derive(Debug, PartialEq)]
enum InterpretResult {
    Done,
    More,
}

/// TODO - https://stackoverflow.com/questions/42097611/how-can-i-better-store-a-string-to-avoid-many-clones
struct CallFrame<'a> {
    function: Function,
    ip: usize,
    base_counter: usize,
    chunk: &'a Chunk,
}

pub struct VM<'a> {
    module: &'a Module,
    frames: Vec<CallFrame<'a>>,
    stack: Vec<Constant>,
    globals: HashMap<String, Constant>,
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

    pub fn interpret(&mut self) -> Result<(), VMError> {
        let function = Function { arity: 0, chunk_index: 0, name: "top".into() };

        self.frames.push(CallFrame {
            function,
            base_counter: 0,
            ip: 0,
            chunk: self.module.chunk(0),
        });

        while self._interpret()? == InterpretResult::More {}
        Ok(())
    }

    fn _interpret(&mut self) -> Result<InterpretResult, VMError> {
        self.current_frame_mut().ip += 1;

        let instr = {
            let frame = self.current_frame_mut();
            frame.chunk.instructions()[frame.ip - 1].clone()
        };

        match instr {
            Instruction::Constant(index) => {
                match self.module.constants()[index].clone() {
                    Constant::Number(n) => self.push(Constant::Number(n)),
                    Constant::String(s) => self.push(Constant::String(s)),
                    _ => panic!("Unknown type!")
                }
            }
            Instruction::True => {
                self.push(Constant::Boolean(true))
            }
            Instruction::False => {
                self.push(Constant::Boolean(false))
            }
            Instruction::None => {}
            Instruction::Add => {
                match (self.pop()?, self.pop()?) {
                    (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::Number(n1 + n2)),
                    (Constant::String(n1), Constant::String(n2)) => self.push(Constant::String(n2.add(n1.as_str()))),
                    (n1, n2) => panic!(format!("Add not implemented for '{:?}' and '{:?}'", n1, n2))
                }
            }
            Instruction::Subtract => {
                match (self.pop()?, self.pop()?) {
                    (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::Number(n2 - n1)),
                    (n1, n2) => panic!(format!("Subtract not implemented for '{:?}' and '{:?}'", n1, n2))
                }
            }
            Instruction::Multiply => {
                match (self.pop()?, self.pop()?) {
                    (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::Number(n2 * n1)),
                    (n1, n2) => panic!(format!("Multiply not implemented for '{:?}' and '{:?}'", n1, n2))
                }
            }
            Instruction::Divide => {
                match (self.pop()?, self.pop()?) {
                    (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::Number(n2 / n1)),
                    (n1, n2) => panic!(format!("Divide not implemented for '{:?}' and '{:?}'", n1, n2))
                }
            }
            Instruction::Modulo => {
                match (self.pop()?, self.pop()?) {
                    (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::Number(n2 % n1)),
                    (n1, n2) => panic!(format!("Modulo not implemented for '{:?}' and '{:?}'", n1, n2))
                }
            }
            Instruction::Pow => {
                match (self.pop()?, self.pop()?) {
                    (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::Number(f64::powf(n2, n1))),
                    (n1, n2) => panic!(format!("Pow not implemented for '{:?}' and '{:?}'", n1, n2))
                }
            }
            Instruction::Not => {
                match self.pop()? {
                    Constant::Boolean(b) => self.push(Constant::Boolean(!b)),
                    n1 => panic!(format!("Not not implemented for '{:?}'", n1))
                }
            }
            Instruction::NotEqual => {
                match (self.pop()?, self.pop()?) {
                    (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::Boolean((n2 - n1).abs() > std::f64::EPSILON)),
                    (n1, n2) => panic!(format!("NotEqual not implemented for '{:?}' and '{:?}'", n1, n2))
                }
            }
            Instruction::EqualEqual => {
                match (self.pop()?, self.pop()?) {
                    (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::Boolean((n2 - n1).abs() < std::f64::EPSILON)),
                    (n1, n2) => panic!(format!("EqualEqual not implemented for '{:?}' and '{:?}'", n1, n2))
                }
            }
            Instruction::Greater => {
                match (self.pop()?, self.pop()?) {
                    (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::Boolean(n2 > n1)),
                    (n1, n2) => panic!(format!("Greater not implemented for '{:?}' and '{:?}'", n1, n2))
                }
            }
            Instruction::GreaterEqual => {
                match (self.pop()?, self.pop()?) {
                    (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::Boolean(n2 >= n1)),
                    (n1, n2) => panic!(format!("GreaterEqual not implemented for '{:?}' and '{:?}'", n1, n2))
                }
            }
            Instruction::Less => {
                match (self.pop()?, self.pop()?) {
                    (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::Boolean(n2 < n1)),
                    (n1, n2) => panic!(format!("Less not implemented for '{:?}' and '{:?}'", n1, n2))
                }
            }
            Instruction::LessEqual => {
                match (self.pop()?, self.pop()?) {
                    (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::Boolean(n2 <= n1)),
                    (n1, n2) => panic!(format!("LessEqual not implemented for '{:?}' and '{:?}'", n1, n2))
                }
            }
            Instruction::GetGlobal(index) => {
                if let Constant::String(key) = self.module.constant(index) {
                    if let Some(constant) = self.globals.get(key.as_str()).cloned() {
                        self.push(constant);
                    } else {
                        return Err(VMError::from(format!("Undefined variable '{}'", key)));
                    }
                } else {
                    panic!("Expected String Constant!");
                }
            }
            Instruction::SetGlobal(index) => {
                if let Constant::String(key) = self.module.constant(index) {
                    let value = self.pop()?;
                    self.globals.insert(key, value);
                } else {
                    panic!("Expected String Constant!");
                }
            }
            Instruction::GetLocal(index) => {
                let index = self.current_frame().base_counter + index;
                self.push(self.stack[index].clone());
            }
            Instruction::SetLocal(index) => {
                let index = self.current_frame().base_counter + index;
                let value = self.peek()?.clone();
                self.stack[index] = value;
            }
            Instruction::JumpIfTrue(to) => {
                if !self.peek()?.is_falsey() {
                    self.current_frame_mut().ip = to;
                }
            }
            Instruction::JumpIfFalse(to) => {
                if self.peek()?.is_falsey() {
                    self.current_frame_mut().ip = to;
                }
            }
            Instruction::Jump(to) => {
                self.current_frame_mut().ip = to;
            }
            Instruction::Call(arity) => {
                self.call(arity)?;
            }
            Instruction::Pop => {
                self.pop();
            }
            Instruction::Return => {
                let res = self.pop();
                let frame = self.frames.pop()
                    .ok_or_else(|| VMError::from("No more call frames!"))?;

                self.stack.split_off(frame.base_counter);

                if self.frames.len() == 0 {
                    return Ok(InterpretResult::Done);
                }
                self.push(res?);
            }
            Instruction::Print => {
                match self.pop()? {
                    Constant::Number(n) => {
                        println!("{}", n)
                    }
                    Constant::String(s) => {
                        println!("{}", s)
                    }
                    Constant::Boolean(b) => {
                        println!("{}", b)
                    }
                    Constant::None => {
                        println!("None")
                    }
                    Constant::Function(_) => {
                        println!("Function?")
                    }
                }
            }
            Instruction::Function(index) => {
                if let Constant::Function(function) = self.module.constant(index) {
                    self.push(Constant::Function(function));
                }
            }
        }
        Ok(InterpretResult::More)
    }

    fn current_frame(&self) -> &CallFrame<'a> {
        match self.frames.last() {
            Some(frame) => frame,
            None => panic!("Expected valid callframe")
        }
    }

    fn current_frame_mut(&mut self) -> &mut CallFrame<'a> {
        match self.frames.last_mut() {
            Some(frame) => frame,
            None => panic!("Expected valid callframe")
        }
    }

    fn push(&mut self, constant: Constant) {
        self.stack.push(constant)
    }

    fn pop(&mut self) -> Result<Constant, VMError> {
        self.stack.pop().ok_or_else(|| VMError::from("Stack Empty!"))
    }

    fn peek(&self) -> Result<&Constant, VMError> {
        self.stack.last().ok_or_else(|| VMError::from("Stack Empty!"))
    }

    fn peek_n(&self, n: usize) -> Result<&Constant, VMError> {
        self.stack.get(self.stack.len() - n - 1).ok_or_else(|| VMError::from("Stack Empty!"))
    }

    fn call(&mut self, arity: usize) -> Result<(), VMError> {
        let callee = self.peek_n(arity)?;
        match callee {
            Constant::Function(callee) => {
                if callee.arity != arity {
                    return Err(VMError::from("function arity doesnt match"));
                } else {
                    self.begin_frame(callee.clone())
                }
            }
            _ => return Err(VMError::from("invalid callee"))
        }
        Ok(())
    }

    fn begin_frame(&mut self, function: Function) {
        self.frames.push(CallFrame {
            ip: 0,
            base_counter: self.stack.len() - function.arity,
            chunk: self.module.chunk(function.chunk_index),
            function,
        })
    }
}