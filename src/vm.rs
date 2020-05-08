use crate::chunk::Instruction;
use crate::constant::Constant;
use crate::error::RuntimeError;
use crate::function::Closure;
use crate::function::Function;
use crate::memory::Distance;
use crate::module::Module;
use std::collections::HashMap;
use std::num::Wrapping;
use string_interner::Sym;

#[derive(Debug, PartialEq)]
pub enum InterpreterResult {
    More,
    Done,
}

#[derive(Debug)]
pub struct CallFrame {
    closure: Closure,
    ip: usize,
    base_counter: usize,
    chunk_index: usize,
}

#[derive(Debug)]
pub struct VM {
    module: Module,
    frames: Vec<CallFrame>,
    stack: Vec<Constant>,
    globals: HashMap<Sym, Constant>,
}

impl VM {
    pub fn new(module: Module) -> Self {
        VM {
            module,
            frames: vec![],
            stack: vec![],
            globals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<(), RuntimeError> {
        let function = Function {
            arity: 0,
            chunk_index: 0,
            name: self.module.get_or_intern("$$top$$"),
        };

        let closure = Closure::new(function);
        self.frames.push(CallFrame {
            closure: closure,
            ip: 0,
            base_counter: 0,
            chunk_index: 0,
        });

        while self._interpret()? == InterpreterResult::More {}
        Ok(())
    }

    pub fn _interpret(&mut self) -> Result<InterpreterResult, RuntimeError> {
        self.current_frame_mut().ip += 1;

        let instruction: &Instruction = {
            let frame = self.current_frame();
            let module = self.module();
            &module.get_chunk(frame.chunk_index).instructions[frame.ip - 1]
        };

        // println!("Stack: {:?}", self.stack);

        match instruction {
            Instruction::Constant(index) => {
                let constant = self.module().constants().get(*index).clone();
                self.push(constant);
            }
            Instruction::True => self.stack.push(Constant::Boolean(true)),
            Instruction::False => self.stack.push(Constant::Boolean(false)),
            Instruction::None => self.stack.push(Constant::None),
            Instruction::Add => match (self.pop()?, self.pop()?) {
                (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::from(n1 + n2)),
                (Constant::String(n1), Constant::String(n2)) => {
                    let mut str1 = self
                        .module
                        .strings
                        .resolve(n2)
                        .expect("Expected a &str")
                        .to_string();
                    let str2 = self.module.strings.resolve(n1).expect("Expected a &str");
                    str1.push_str(str2);

                    let sym = self.module.get_or_intern(&str1.as_str());
                    self.push(Constant::String(sym))
                }
                (Constant::Array(e2), Constant::Array(e1)) => {
                    let v = [&e1[..], &e2[..]].concat();
                    self.push(Constant::Array(v));
                }
                (n1, n2) => panic!(format!("Add not implemented for '{:?}' and '{:?}'", n1, n2)),
            },
            Instruction::Subtract => match (self.pop()?, self.pop()?) {
                (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::from(n2 - n1)),
                (n1, n2) => panic!(format!(
                    "Subtract not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::Negate => match self.pop()? {
                Constant::Number(n) => self.push(Constant::from(Distance::from(-1.0) * n)),
                n => panic!(format!("Negate not implemented for '{:?}''", n)),
            },
            Instruction::Multiply => match (self.pop()?, self.pop()?) {
                (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::from(n2 * n1)),
                (n1, n2) => panic!(format!(
                    "Multiply not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::Divide => match (self.pop()?, self.pop()?) {
                (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::from(n2 / n1)),
                (n1, n2) => panic!(format!(
                    "Divide not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::Modulo => match (self.pop()?, self.pop()?) {
                (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::from(n2 % n1)),
                (n1, n2) => panic!(format!(
                    "Modulo not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::Pow => match (self.pop()?, self.pop()?) {
                (Constant::Number(n1), Constant::Number(n2)) => self.push(Constant::from(
                    f64::powf(Into::<f64>::into(n2), Into::<f64>::into(n1)),
                )),
                (n1, n2) => panic!(format!("Pow not implemented for '{:?}' and '{:?}'", n1, n2)),
            },
            Instruction::Not => match self.pop()? {
                Constant::Boolean(b) => self.push(Constant::Boolean(!b)),
                n1 => panic!(format!("Not not implemented for '{:?}'", n1)),
            },
            Instruction::NotEqual => match (self.pop()?, self.pop()?) {
                (Constant::Number(n1), Constant::Number(n2)) => {
                    let res = n2 - n1;
                    let nabs = if res < 0.0 { res * -1.0 } else { res };
                    self.push(Constant::Boolean(nabs > std::f64::EPSILON))
                }
                (n1, n2) => panic!(format!(
                    "NotEqual not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::EqualEqual => match (self.pop()?, self.pop()?) {
                (Constant::Number(n1), Constant::Number(n2)) => {
                    self.push(Constant::Boolean((n2 - n1).abs() < std::f64::EPSILON))
                }
                (Constant::Array(a1), Constant::Array(a2)) => {
                    self.push(Constant::Boolean(a1 == a2))
                }
                (_, _) => self.push(Constant::Boolean(false)),
            },
            Instruction::Greater => match (self.pop()?, self.pop()?) {
                (Constant::Number(n1), Constant::Number(n2)) => {
                    self.push(Constant::Boolean(n2 > n1))
                }
                (n1, n2) => panic!(format!(
                    "Greater not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::GreaterEqual => match (self.pop()?, self.pop()?) {
                (Constant::Number(n1), Constant::Number(n2)) => {
                    self.push(Constant::Boolean(n2 >= n1))
                }
                (n1, n2) => panic!(format!(
                    "GreaterEqual not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::Less => match (self.pop()?, self.pop()?) {
                (Constant::Number(n1), Constant::Number(n2)) => {
                    self.push(Constant::Boolean(n2 < n1))
                }
                (n1, n2) => panic!(format!(
                    "Less not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::LessEqual => match (self.pop()?, self.pop()?) {
                (Constant::Number(n1), Constant::Number(n2)) => {
                    self.push(Constant::Boolean(n2 <= n1))
                }
                (n1, n2) => panic!(format!(
                    "LessEqual not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::JumpIfTrue(to) => {
                if let Constant::Boolean(true) = self.peek()? {
                    self.current_frame_mut().ip = *to;
                }
            }
            Instruction::JumpIfFalse(to) => {
                if let Constant::Boolean(false) = self.peek()? {
                    self.current_frame_mut().ip = *to;
                }
            }
            Instruction::Jump(to) => {
                self.current_frame_mut().ip = *to;
            }
            Instruction::GetGlobal(index) => {
                if let Constant::String(key) = self.module.constants.get(*index) {
                    if let Some(constant) = self.globals.get(key) {
                        let c = constant.clone();
                        self.push(c);
                    } else {
                        return Err(RuntimeError::UndefinedVariable);
                    }
                } else {
                    return Err(RuntimeError::ExpectedStringConstant);
                }
            }
            Instruction::GetLocal(index) => {
                let index = self.current_frame().base_counter + index;
                self.push(self.stack[index].clone());
            }
            Instruction::GetUpvalue(index) => {}
            Instruction::SetGlobal(index) => {
                let module = self.module();
                if let Constant::String(key) = module.constants.get(*index) {
                    let _key = key.clone();
                    let value = self.pop()?;
                    self.globals.insert(_key, value);
                } else {
                    panic!("Expected String Constant!");
                }
            }
            Instruction::SetLocal(index) => {
                let index = self.current_frame().base_counter + index;
                let value = self.peek()?.clone();
                self.stack[index] = value;
            }
            Instruction::SetUpvalue(index) => {}
            Instruction::Call(arity) => {
                let a = *arity;
                self.call(a)?;
            }
            Instruction::Closure(index) => {
                let module = &self.module();
                if let Constant::Closure(closure) = module.constants.get(*index) {
                    let constant = Constant::Closure(closure.clone());
                    self.push(constant);
                }
            }
            Instruction::Pop => {
                self.pop()?;
            }
            Instruction::List(length) => {
                let mut elements = Vec::new();
                for _ in 0..*length {
                    elements.push(self.pop()?);
                }
                elements.reverse();
                self.stack.push(Constant::Array(elements))
            }
            Instruction::Slice => {
                let step = match self.pop()? {
                    Constant::Number(n) => Some(n),
                    Constant::None => None,
                    _ => return Err(RuntimeError::ExpectedNumber),
                };

                let stop = match self.pop()? {
                    Constant::Number(n) => Some(n),
                    Constant::None => None,
                    _ => return Err(RuntimeError::ExpectedNumber),
                };

                let start = match self.pop()? {
                    Constant::Number(n) => Some(n),
                    Constant::None => None,
                    _ => return Err(RuntimeError::ExpectedNumber),
                };

                let mut arr: Vec<Constant> = match self.pop()? {
                    Constant::Array(elements) => elements,
                    _ => return Err(RuntimeError::ExpectedArray),
                };

                let mut res: Vec<Constant> = vec![];

                let a = match start {
                    Some(t) => Into::<f64>::into(t) as usize,
                    None => 0,
                };

                let b = match stop {
                    Some(t) => Into::<f64>::into(t) as usize,
                    None => arr.len(),
                };

                let c = match step {
                    Some(t) => Into::<f64>::into(t) as isize,
                    None => 1,
                };

                if c > 0 {
                    for i in (a..b).step_by(c as usize) {
                        res.push(arr[i].clone());
                    }
                } else {
                    arr.reverse();
                    for i in (a..b).step_by(-c as usize) {
                        res.push(arr[i].clone());
                    }
                };
                self.push(Constant::Array(res));
            }
            Instruction::Index => {
                let index: isize = match self.pop()? {
                    Constant::Number(n) => Into::<f64>::into(n) as isize,
                    _ => return Err(RuntimeError::ExpectedNumber),
                };

                match self.pop()? {
                    Constant::Array(elements) => {
                        if index < 0 {
                            if (-index as usize) >= elements.len() {
                                return Err(RuntimeError::IndexOutOfBounds);
                            } else {
                                let len = elements.len();
                                self.push(elements[len - (-index as usize)].clone());
                            }
                        } else {
                            if (index as usize) >= elements.len() {
                                return Err(RuntimeError::IndexOutOfBounds);
                            } else {
                                self.push(elements[index as usize].clone());
                            }
                        }
                    }
                    _ => return Err(RuntimeError::ExpectedNumber),
                };
            }
            Instruction::Print => {
                self.print_constant()?;
            }
            Instruction::Return => {
                let res = self.pop();
                let frame = self
                    .frames
                    .pop()
                    .ok_or_else(|| RuntimeError::CallFrameEmpty)?;

                self.stack.split_off(frame.base_counter);

                if self.frames.is_empty() {
                    return Ok(InterpreterResult::Done);
                }
                self.push(res?);
            }
        };

        Ok(InterpreterResult::More)
    }

    fn current_frame(&self) -> &CallFrame {
        self.frames.last().expect("Expect &mut Callframe to exist")
    }

    fn current_frame_mut(&mut self) -> &mut CallFrame {
        self.frames
            .last_mut()
            .expect("Expect &mut Callframe to exist")
    }

    fn module(&self) -> &Module {
        &self.module
    }

    fn pop(&mut self) -> Result<Constant, RuntimeError> {
        self.stack.pop().ok_or_else(|| RuntimeError::StackEmpty)
    }

    fn peek(&self) -> Result<&Constant, RuntimeError> {
        self.stack.last().ok_or_else(|| RuntimeError::StackEmpty)
    }

    fn peek_n(&self, n: usize) -> Result<&Constant, RuntimeError> {
        self.stack
            .get(self.stack.len() - n - 1)
            .ok_or_else(|| RuntimeError::StackEmpty)
    }

    fn push(&mut self, constant: Constant) {
        self.stack.push(constant)
    }

    fn print_constant(&mut self) -> Result<(), RuntimeError> {
        let constant = self.pop()?;
        self.constant_to_string(&constant)?;
        println!("");
        Ok(())
    }

    fn constant_to_string(&mut self, constant: &Constant) -> Result<(), RuntimeError> {
        match constant {
            Constant::Number(n) => {
                let f: f64 = Into::<f64>::into(n);
                print!("{}", f);
            }
            Constant::String(s) => print!(
                "{}",
                self.module
                    .strings
                    .resolve(*s)
                    .expect("Expected string in interner")
            ),
            Constant::Boolean(b) => print!("{}", b),
            Constant::None => print!("none"),
            Constant::Array(elements) => {
                print!("[");
                for (i, e) in elements.iter().enumerate() {
                    self.constant_to_string(e)?;
                    if i != elements.len() - 1 {
                        print!(", ");
                    }
                }
                print!("]");
            }
            Constant::Closure(_) => print!("Closure?"),
            Constant::Class(_) => print!("Class?"),
        };
        Ok(())
    }

    fn call(&mut self, arity: usize) -> Result<(), RuntimeError> {
        let callee = self.peek_n(arity)?;
        match callee {
            Constant::Closure(callee) => {
                if callee.function.arity != arity {
                    return Err(RuntimeError::IncorrectArity);
                }
                let c = callee.clone();
                self.begin_frame(c);
            }
            _ => return Err(RuntimeError::ExpectedCallee),
        }
        Ok(())
    }

    fn begin_frame(&mut self, closure: Closure) {
        let arity = closure.function.arity;
        let index = closure.function.chunk_index;
        self.frames.push(CallFrame {
            closure: closure,
            ip: 0,
            base_counter: self.stack.len() - arity,
            chunk_index: index,
        });
    }
}
