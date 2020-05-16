use crate::bytecode::chunk::Instruction;
use crate::bytecode::constant::Constant;
use crate::bytecode::module::Module;
use crate::vm::class::BoundMethod;
use crate::vm::class::Class;
use crate::vm::class::Instance;
use crate::vm::error::RuntimeError;
use crate::vm::function::Closure;
use crate::vm::function::Function;
use crate::vm::function::NativeFunction;
use crate::vm::gc;
use crate::vm::managed::Managed;
use crate::vm::memory::Upvalue;
use crate::vm::memory::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use string_interner::Sym;

#[derive(Debug, PartialEq)]
pub enum InterpreterResult {
    More,
    Done,
}

#[derive(Debug, Copy, Clone)]
pub struct CallFrame {
    closure: Managed<Closure>,
    ip: usize,
    base_counter: usize,
    chunk_index: usize,
}

#[derive(Debug)]
pub struct VM<'a> {
    module: &'a mut Module,
    frames: Vec<CallFrame>,
    stack: Vec<Value>,
    globals: HashMap<Sym, Value>,
    upvalues: Vec<Managed<RefCell<Upvalue>>>,
}

impl<'a> VM<'a> {
    pub fn new(module: &'a mut Module) -> Self {
        VM {
            module,
            frames: vec![],
            stack: vec![],
            globals: HashMap::new(),
            upvalues: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn set_native_fn(
        &mut self,
        identifier: &str,
        arity: usize,
        code: fn(&[Value]) -> Result<Value, RuntimeError>,
    ) {
        let name = self.module.strings.get_or_intern(identifier.to_string());
        let native = gc::manage(NativeFunction {
            name: name,
            arity: arity,
            code: code,
        });

        self.globals.insert(name, Value::NativeFunction(native));
    }

    pub fn interpret(&mut self) -> Result<(), RuntimeError> {
        let function = Function {
            arity: 0,
            chunk_index: 0,
            name: self.module.get_or_intern(""),
        };

        let closure = Closure {
            function: gc::manage(function),
            upvalues: vec![],
        };

        self.frames.push(CallFrame {
            closure: gc::manage(closure),
            ip: 0,
            base_counter: 0,
            chunk_index: 0,
        });

        while self._interpret()? == InterpreterResult::More {}
        Ok(())
    }

    pub fn _interpret(&mut self) -> Result<InterpreterResult, RuntimeError> {
        self.current_frame_mut().ip += 1;

        let instruction: Instruction = {
            let frame = self.current_frame();
            self.module.get_chunk(frame.chunk_index).instructions[frame.ip - 1]
        };

        // println!("instruction: {:?}", instruction);
        // println!("stack: {:?}", self.stack);
        // println!("globals:");
        // for (key, value) in self.globals.iter() {
        //     println!("\t{:?}: {:?}", key, value);
        // }

        match instruction {
            Instruction::Constant(index) => {
                let constant = self.module.constants.get(index);
                match constant {
                    Constant::Number(dist) => {
                        let f64_fr_dist = Into::<f64>::into(*dist);
                        self.push(Value::Number(f64_fr_dist));
                    }
                    Constant::String(sym) => {
                        let string = *sym;
                        self.push(Value::String(string))
                    }
                    Constant::Closure(_) => (),
                    Constant::Class(_) => (),
                };
            }
            Instruction::True => self.push(Value::Boolean(true)),
            Instruction::False => self.push(Value::Boolean(false)),
            Instruction::None => self.push(Value::None),
            Instruction::Add => match (self.pop()?, self.pop()?) {
                (Value::Number(n1), Value::Number(n2)) => self.push(Value::Number(n1 + n2)),
                (Value::String(n1), Value::String(n2)) => {
                    let mut str1 = self
                        .module
                        .strings
                        .resolve(n2)
                        .expect("Expected a &str")
                        .to_string();
                    let str2 = self.module.strings.resolve(n1).expect("Expected a &str");
                    str1.push_str(str2);

                    let sym = self.module.get_or_intern(&str1.as_str());
                    self.push(Value::String(sym))
                }
                (other, Value::String(n1)) => {
                    let mut str1 = self
                        .module
                        .strings
                        .resolve(n1)
                        .expect("Expected a &str")
                        .to_string();
                    str1.push_str(self.value_to_string(&other, false)?.as_str());

                    let sym = self.module.get_or_intern(&str1.as_str());
                    self.push(Value::String(sym))
                }
                (Value::Array(e2), Value::Array(e1)) => {
                    let v = [&e1[..], &e2[..]].concat();
                    self.push(Value::Array(gc::manage(v)));
                }
                (n1, n2) => panic!(format!("Add not implemented for '{:?}' and '{:?}'", n1, n2)),
            },
            Instruction::Subtract => match (self.pop()?, self.pop()?) {
                (Value::Number(n1), Value::Number(n2)) => self.push(Value::Number(n2 - n1)),
                (n1, n2) => panic!(format!(
                    "Subtract not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::Negate => match self.pop()? {
                Value::Number(n) => self.push(Value::Number(-1.0 * n)),
                n => panic!(format!("Negate not implemented for '{:?}''", n)),
            },
            Instruction::Multiply => match (self.pop()?, self.pop()?) {
                (Value::Number(n1), Value::Number(n2)) => self.push(Value::Number(n2 * n1)),
                (n1, n2) => panic!(format!(
                    "Multiply not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::Divide => match (self.pop()?, self.pop()?) {
                (Value::Number(n1), Value::Number(n2)) => self.push(Value::Number(n2 / n1)),
                (n1, n2) => panic!(format!(
                    "Divide not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::Modulo => match (self.pop()?, self.pop()?) {
                (Value::Number(n1), Value::Number(n2)) => self.push(Value::Number(n2 % n1)),
                (n1, n2) => panic!(format!(
                    "Modulo not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::Pow => match (self.pop()?, self.pop()?) {
                (Value::Number(n1), Value::Number(n2)) => self.push(Value::Number(f64::powf(
                    Into::<f64>::into(n2),
                    Into::<f64>::into(n1),
                ))),
                (n1, n2) => panic!(format!("Pow not implemented for '{:?}' and '{:?}'", n1, n2)),
            },
            Instruction::Not => match self.pop()? {
                Value::Boolean(b) => self.push(Value::Boolean(!b)),
                n1 => panic!(format!("Not not implemented for '{:?}'", n1)),
            },
            Instruction::NotEqual => match (self.pop()?, self.pop()?) {
                (Value::Number(n1), Value::Number(n2)) => {
                    let res = n2 - n1;
                    let nabs = if res < 0.0 { res * -1.0 } else { res };
                    self.push(Value::Boolean(nabs > std::f64::EPSILON))
                }
                (n1, n2) => panic!(format!(
                    "NotEqual not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::EqualEqual => match (self.pop()?, self.pop()?) {
                (Value::Number(n1), Value::Number(n2)) => {
                    self.push(Value::Boolean((n2 - n1).abs() < std::f64::EPSILON))
                }
                (Value::Array(a1), Value::Array(a2)) => self.push(Value::Boolean(a1 == a2)),
                (_, _) => self.push(Value::Boolean(false)),
            },
            Instruction::Greater => match (self.pop()?, self.pop()?) {
                (Value::Number(n1), Value::Number(n2)) => self.push(Value::Boolean(n2 > n1)),
                (n1, n2) => panic!(format!(
                    "Greater not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::GreaterEqual => match (self.pop()?, self.pop()?) {
                (Value::Number(n1), Value::Number(n2)) => self.push(Value::Boolean(n2 >= n1)),
                (n1, n2) => panic!(format!(
                    "GreaterEqual not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::Less => match (self.pop()?, self.pop()?) {
                (Value::Number(n1), Value::Number(n2)) => self.push(Value::Boolean(n2 < n1)),
                (n1, n2) => panic!(format!(
                    "Less not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::LessEqual => match (self.pop()?, self.pop()?) {
                (Value::Number(n1), Value::Number(n2)) => self.push(Value::Boolean(n2 <= n1)),
                (n1, n2) => panic!(format!(
                    "LessEqual not implemented for '{:?}' and '{:?}'",
                    n1, n2
                )),
            },
            Instruction::JumpIfTrue(to) => {
                if let Value::Boolean(true) = self.peek()? {
                    self.current_frame_mut().ip = to;
                }
            }
            Instruction::JumpIfFalse(to) => {
                if let Value::Boolean(false) = self.peek()? {
                    self.current_frame_mut().ip = to;
                }
            }
            Instruction::Jump(to) => {
                self.current_frame_mut().ip = to;
            }
            Instruction::GetGlobal(index) => {
                if let Constant::String(key) = self.module.constants.get(index) {
                    let glob_key = self.globals.get(key).cloned();
                    if let Some(constant) = glob_key {
                        self.push(constant);
                    } else {
                        println!("Variable: {:?}", key);
                        return Err(RuntimeError::UndefinedVariable);
                    }
                } else {
                    return Err(RuntimeError::ExpectedStringConstant);
                }
            }
            Instruction::GetLocal(index) => {
                let index = self.current_frame().base_counter + index;
                self.push(self.stack[index]);
            }
            Instruction::GetUpvalue(index) => {
                let upvalue = self.current_frame().closure.upvalues[index];
                let value = match *upvalue.borrow() {
                    Upvalue::Open(index) => self.stack[index],
                    Upvalue::Closed(value) => value,
                };
                self.push(value);
            }
            Instruction::SetGlobal(index) => {
                let value = *self.peek()?;
                if let Constant::String(key) = self.module.constants.get(index) {
                    self.globals.insert(*key, value);
                    self.pop()?;
                } else {
                    panic!("Expected String Constant!");
                }
            }
            Instruction::SetLocal(index) => {
                let index = self.current_frame().base_counter + index;
                let value = *self.peek()?;
                self.stack[index] = value;
            }
            Instruction::SetUpvalue(index) => {
                let value = self.peek()?;
                let upvalue = self.current_frame().closure.upvalues[index];
                match &mut *upvalue.borrow_mut() {
                    Upvalue::Closed(v) => *v = *value,
                    Upvalue::Open(index) => self.stack[*index] = *value,
                };
            }
            Instruction::Call(arity) => {
                self.call(arity)?;
            }
            Instruction::Closure(index) => {
                if let Constant::Closure(closure) = self.module.get_constant(index) {
                    let mut upvalues: Vec<Managed<RefCell<Upvalue>>> = vec![];

                    for u in &closure.upvalues {
                        if u.is_local {
                            let frame = &self.frames[self.frames.len() - 1];
                            let base = frame.base_counter;
                            let index = base + index;

                            let mut upvalue: Option<Managed<RefCell<Upvalue>>> = None;
                            for managed in self.upvalues.iter().rev() {
                                if (*managed).borrow().is_open(index) {
                                    upvalue = Some(*managed);
                                }
                            }

                            match upvalue {
                                Some(up) => upvalues.push(up),
                                None => {
                                    let managed = gc::manage(RefCell::new(Upvalue::Open(index)));
                                    self.upvalues.push(managed.clone());
                                    upvalues.push(managed);
                                }
                            }
                        } else {
                            let frame = self.frames[self.frames.len() - 1];
                            upvalues.push(frame.closure.upvalues[index]);
                        }
                    }

                    let function = gc::manage(Function {
                        name: closure.function.name,
                        chunk_index: closure.function.chunk_index,
                        arity: closure.function.arity,
                    });

                    let closure = gc::manage(Closure {
                        function: function,
                        upvalues: upvalues,
                    });

                    self.push(Value::Closure(closure));
                }
            }
            Instruction::Class(index) => {
                if let Constant::Class(class) = self.module.constants.get(index) {
                    let class = Class {
                        name: class.name,
                        methods: HashMap::new(),
                    };
                    self.push(Value::Class(gc::manage(RefCell::new(class))));
                } else {
                    return Err(RuntimeError::UnexpectedConstant);
                }
            }
            Instruction::GetProperty(index) => {
                if let Value::Instance(instance) = self.pop()? {
                    if let Constant::String(property) = self.module.get_constant(index) {
                        if let Some(value) = instance.borrow().fields.get(property) {
                            self.push(*value);
                        } else if let Some(method) =
                            instance.borrow().class.borrow().methods.get(property)
                        {
                            let bounded = BoundMethod {
                                receiver: Value::Instance(instance),
                                method: *method,
                            };
                            let constant = Value::BoundMethod(gc::manage(bounded));
                            self.push(constant);
                        } else {
                            return Err(RuntimeError::UndefinedProperty);
                        }
                    } else {
                        return Err(RuntimeError::ExpectedInstance);
                    }
                }
            }
            Instruction::SetProperty(index) => {
                if let Constant::String(property) = self.module.get_constant(index) {
                    if let Value::Instance(instance) = self.peek_n(1)? {
                        instance
                            .borrow_mut()
                            .fields
                            .insert(property.clone(), self.peek()?.clone());

                        let value = self.pop()?;
                        self.pop()?;
                        self.push(value);
                    } else {
                        return Err(RuntimeError::UnexpectedValue);
                    }
                } else {
                    return Err(RuntimeError::UnexpectedConstant);
                }
            }
            Instruction::Method => {
                // println!("Method Stack: {:?}", self.stack);
                // if let Value::Closure(closure) = self.peek()? {
                //     if let Value::Class(class) = self.peek_n(1)? {
                //         let name = function.name;
                //         class.borrow_mut().methods.insert(name, *function);
                //         self.pop()?;
                //     } else {
                //         return Err(RuntimeError::ExpectedClass);
                //     }
                // } else {
                //     return Err(RuntimeError::ExpectedCallee);
                // }
            }
            Instruction::CloseUpvalue => {
                let index = self.stack.len() - 1;
                let value = self.stack[index]; //TODO Result
                for root in &self.upvalues {
                    if root.borrow().is_open(index) {
                        root.replace(Upvalue::Closed(value));
                    }
                }
                self.stack.pop().ok_or(RuntimeError::StackEmpty)?;
            }
            Instruction::Pop => {
                self.pop()?;
            }
            Instruction::List(length) => {
                let mut elements = Vec::new();
                for _ in 0..length {
                    let e = self.pop()?;
                    elements.push(e);
                }
                elements.reverse();
                self.push(Value::Array(gc::manage(elements)))
            }
            Instruction::Slice => {
                let step = match self.pop()? {
                    Value::Number(n) => Some(n),
                    Value::None => None,
                    _ => return Err(RuntimeError::ExpectedNumber),
                };

                let stop = match self.pop()? {
                    Value::Number(n) => Some(n),
                    Value::None => None,
                    _ => return Err(RuntimeError::ExpectedNumber),
                };

                let start = match self.pop()? {
                    Value::Number(n) => Some(n),
                    Value::None => None,
                    _ => return Err(RuntimeError::ExpectedNumber),
                };

                let mut arr: Vec<Value> = match self.pop()? {
                    Value::Array(elements) => (*elements).clone(),
                    _ => return Err(RuntimeError::ExpectedArray),
                };

                let mut res: Vec<Value> = vec![];

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
                self.push(Value::Array(gc::manage(res)));
            }
            Instruction::Range => {
                let step = match self.pop()? {
                    Value::Number(n) => Some(n),
                    Value::None => None,
                    _ => return Err(RuntimeError::ExpectedNumber),
                };

                let stop = match self.pop()? {
                    Value::Number(n) => Into::<f64>::into(n) as isize,
                    _ => return Err(RuntimeError::ExpectedNumber),
                };

                let start = match self.pop()? {
                    Value::Number(n) => Into::<f64>::into(n) as isize,
                    _ => return Err(RuntimeError::ExpectedNumber),
                };

                let mut res: Vec<Value> = vec![];

                let c = match step {
                    Some(t) => Into::<f64>::into(t) as isize,
                    None => 1,
                };

                if c > 0 {
                    for i in (start..stop).step_by(c as usize) {
                        res.push(Value::Number(i as f64));
                    }
                } else {
                    for i in (start..stop).step_by(-c as usize) {
                        res.push(Value::Number(i as f64));
                    }
                    res.reverse();
                };
                self.push(Value::Array(gc::manage(res)));
            }
            Instruction::Index => {
                let index: isize = match self.pop()? {
                    Value::Number(n) => Into::<f64>::into(n) as isize,
                    _ => return Err(RuntimeError::ExpectedNumber),
                };

                match self.pop()? {
                    Value::Array(elements) => {
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
            Instruction::Print(n) => {
                let mut elements = self.pop_n(n)?;
                elements.reverse();
                for e in elements {
                    self.print_value(&e)?;
                    print!(" ");
                }
                println!("");
            }
            Instruction::Return => {
                let res = self.pop();
                let frame = self
                    .frames
                    .pop()
                    .ok_or_else(|| RuntimeError::CallFrameEmpty)?;

                self.stack.truncate(frame.base_counter);

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

    fn pop(&mut self) -> Result<Value, RuntimeError> {
        self.stack.pop().ok_or_else(|| RuntimeError::StackEmpty)
    }

    fn pop_n(&mut self, n: usize) -> Result<Vec<Value>, RuntimeError> {
        let mut result = vec![];
        while result.len() < n {
            result.push(self.pop()?);
        }

        Ok(result)
    }

    fn peek(&self) -> Result<&Value, RuntimeError> {
        self.stack.last().ok_or_else(|| RuntimeError::StackEmpty)
    }

    fn peek_n(&self, n: usize) -> Result<&Value, RuntimeError> {
        let len = self.stack.len();
        self.stack
            .get(len - n - 1)
            .ok_or_else(|| RuntimeError::StackEmpty)
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    fn print_value(&mut self, value: &Value) -> Result<(), RuntimeError> {
        let res = self.value_to_string(value, false)?;
        print!("{}", res);
        Ok(())
    }

    fn value_to_string(&mut self, value: &Value, nested: bool) -> Result<String, RuntimeError> {
        match value {
            Value::Number(n) => Ok(format!("{}", n)),
            Value::String(s) => {
                let string = self
                    .module
                    .strings
                    .resolve(*s)
                    .expect("Expected string in interner");
                if nested {
                    Ok(format!("\'{}\'", string))
                } else {
                    Ok(format!("{}", string))
                }
            }
            Value::Boolean(b) => Ok(format!("{}", b)),
            Value::None => Ok(format!("none")),
            Value::Array(elements) => {
                let mut res: String = "[".to_string();
                for (i, e) in elements.iter().enumerate() {
                    let next = self.value_to_string(e, true)?;
                    res.push_str(next.as_str());
                    if i != elements.len() - 1 {
                        res.push_str(", ");
                    }
                }
                res.push_str("]");
                Ok(res)
            }
            Value::Closure(_) => Ok(format!("Closure?")),
            Value::NativeFunction(_) => Ok(format!("NativeFunction?")),
            Value::Class(class) => Ok(format!(
                "class '{}'\n\tmethods: {:?}",
                self.module.strings.resolve(class.borrow().name).expect(""),
                class
                    .borrow()
                    .methods
                    .keys()
                    .map(|key| self.module.strings.resolve(*key).expect(""))
                    .collect::<Vec<&str>>()
            )),
            Value::Instance(_) => Ok(format!("instance",)),
            Value::BoundMethod(bounded) => Ok(format!(
                "bound_method: <{:?}>",
                self.module.strings.resolve((*(**bounded).method).name)
            )),
        }
    }

    fn call(&mut self, arity: usize) -> Result<(), RuntimeError> {
        let callee = *self.peek_n(arity)?;
        match callee {
            Value::Closure(callee) => {
                if callee.function.arity != arity {
                    println!(
                        "arity: {:?} ; callee: {:?}, on {:?}",
                        arity, callee.function.arity, callee.function.name
                    );
                    return Err(RuntimeError::IncorrectArity);
                }
                self.begin_frame(callee);
            }
            Value::NativeFunction(callee) => {
                if callee.arity != arity {
                    return Err(RuntimeError::IncorrectArity);
                }
                let mut args = self.pop_n(arity)?;
                args.reverse();
                self.pop()?;
                let result = (callee.code)(&args);
                self.push(result?);
            }
            Value::Class(class) => {
                if arity > 0 {
                    unimplemented!("Calling a class with arguments is not yet supported");
                }
                self.pop()?; //TODO Temporary, remove when arguments are supported

                let instance = Instance {
                    class: class,
                    fields: HashMap::new(),
                };

                self.push(Value::Instance(gc::manage(RefCell::new(instance))));
            }
            // Value::BoundMethod(callee) => {
            //     if (*(*callee).method).arity != arity {
            //         println!(
            //             "arity: {:?} ; callee: {:?}, on {:?}",
            //             arity,
            //             (*(*callee).method).arity,
            //             (*(*callee).method).name
            //         );
            //         return Err(RuntimeError::IncorrectArity);
            //     }

            //     let len = self.stack.len();
            //     println!("callee.receiver: {:?}", (*callee).receiver);
            //     println!("stack: {:?}", self.stack);
            //     self.stack[len - arity - 1] = (*callee).receiver;
            //     // self.stack.push((*callee).receiver);
            //     println!("stack: {:?}", self.stack);

            //     self.begin_frame((*callee).method);
            // }
            _ => return Err(RuntimeError::ExpectedCallee),
        }
        Ok(())
    }

    fn begin_frame(&mut self, closure: Managed<Closure>) {
        let arity = closure.function.arity;
        let index = closure.function.chunk_index;
        self.frames.push(CallFrame {
            closure: closure,
            ip: 0,
            base_counter: self.stack.len() - arity - 1, // Care of this -1 here...
            chunk_index: index,
        });
    }
}
