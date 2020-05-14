use crate::chunk::Instruction;
use crate::class::Class;
use crate::class::Instance;
use crate::constant::Constant;
use crate::error::RuntimeError;
use crate::function::Function;
use crate::function::NativeFunction;
use crate::managed::make_managed;
use crate::managed::Managed;
use crate::memory::Value;
use crate::module::Module;
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
    function: Managed<Function>,
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
}

impl<'a> VM<'a> {
    pub fn new(module: &'a mut Module) -> Self {
        VM {
            module,
            frames: vec![],
            stack: vec![],
            globals: HashMap::new(),
        }
    }

    pub fn set_native_fn(
        &mut self,
        identifier: &str,
        arity: usize,
        code: fn(&[Value]) -> Result<Value, RuntimeError>,
    ) {
        let sym = self.module.strings.get_or_intern(identifier.to_string());
        let native_function = make_managed(NativeFunction {
            name: sym,
            arity: arity,
            code: code,
        });

        self.globals
            .insert(sym, Value::NativeFunction(native_function));
    }

    pub fn interpret(&mut self) -> Result<(), RuntimeError> {
        let function = Function {
            arity: 0,
            chunk_index: 0,
            name: self.module.get_or_intern("$$top$$"),
        };

        self.frames.push(CallFrame {
            function: make_managed(function),
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
            self.module.get_chunk(frame.chunk_index).instructions[frame.ip - 1].clone()
        };

        println!("instruction: {:?}", instruction);

        match instruction {
            Instruction::Constant(index) => {
                let constant = self.module.constants.get(index).clone();
                match constant {
                    Constant::Number(n) => self.push(Value::Number(Into::<f64>::into(n))),
                    Constant::String(sym) => self.push(Value::String(sym)),
                    Constant::Function(_) => (),
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
                    self.push(Value::Array(make_managed(v)));
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
            Instruction::SetGlobal(index) => {
                if let Constant::String(key) = self.module.constants.get(index) {
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
            Instruction::Call(arity) => {
                self.call(arity)?;
            }
            Instruction::Function(index) => {
                let module = &self.module;
                if let Constant::Function(function) = module.constants.get(index) {
                    let constant = Value::Function(make_managed(function.clone()));
                    self.push(constant);
                }
            }
            Instruction::Class(index) => {
                if let Constant::Class(class) = self.module.constants.get(index) {
                    let class = Class {
                        name: class.name.clone(),
                        methods: HashMap::new(),
                    };
                    self.push(Value::Class(make_managed(RefCell::new(class))));
                } else {
                    return Err(RuntimeError::UnexpectedConstant);
                }
            }
            Instruction::GetProperty(index) => {
                if let Value::Instance(instance) = self.pop()? {
                    if let Constant::String(property) = self.module.get_constant(index) {
                        if let Some(value) = instance.borrow().fields.get(property) {
                            self.push(value.clone());
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
            Instruction::Method => {}
            Instruction::Pop => {
                self.pop()?;
            }
            Instruction::List(length) => {
                let mut elements = Vec::new();
                for _ in 0..length {
                    elements.push(self.pop()?);
                }
                elements.reverse();
                self.push(Value::Array(make_managed(elements)))
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
                self.push(Value::Array(make_managed(res)));
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
                self.push(Value::Array(make_managed(res)));
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
            Instruction::Print(ref n) => {
                let no_elem = *n;
                let mut elements = self.pop_n(no_elem)?;
                elements.reverse();
                for e in elements {
                    self.print_value(e)?;
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

    fn print_value(&mut self, value: Value) -> Result<(), RuntimeError> {
        let res = self.value_to_string(&value, false)?;
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
            Value::Function(_) => Ok(format!("Function?")),
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
        }
    }

    fn call(&mut self, arity: usize) -> Result<(), RuntimeError> {
        let callee = *self.peek_n(arity)?;
        match callee {
            Value::Function(callee) => {
                if callee.arity != arity {
                    return Err(RuntimeError::IncorrectArity);
                }
                self.begin_frame(callee);
            }
            Value::NativeFunction(callee) => {
                println!("arity: {:?} ; callee: {:?}", arity, (*callee).arity);
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

                self.push(Value::Instance(make_managed(RefCell::new(instance))));
            }
            _ => return Err(RuntimeError::ExpectedCallee),
        }
        Ok(())
    }

    fn begin_frame(&mut self, function: Managed<Function>) {
        let arity = function.arity;
        let index = function.chunk_index;
        self.frames.push(CallFrame {
            function: function,
            ip: 0,
            base_counter: self.stack.len() - arity,
            chunk_index: index,
        });
    }
}
