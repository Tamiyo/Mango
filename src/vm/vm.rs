/// The VM, the beating heart of the language.
///
/// The VM takes in a set of bytecode and executes the bytecode.
/// Bytecode execution "rules" are defined here as well, though it
/// may be better to move them out to a seperate stage at some point.
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
use crate::vm::gc::gc;
use crate::vm::gc::managed::Gc;
use crate::vm::gc::managed::Root;
use crate::vm::gc::managed::UniqueRoot;
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

#[derive(Debug, Clone)]
pub struct CallFrame {
    closure: Root<Closure>,
    ip: usize,
    base_counter: usize,
    chunk_index: usize,
}

#[derive(Debug)]
pub struct VM<'a> {
    module: &'a mut Module,
    frames: Vec<CallFrame>,
    stack: UniqueRoot<Vec<Value>>,
    globals: UniqueRoot<HashMap<Sym, Value>>,
    upvalues: Vec<Root<RefCell<Upvalue>>>,
    init_string: Sym,
}

impl<'a> VM<'a> {
    pub fn new(module: &'a mut Module) -> Self {
        // TODO :- Move to another init file
        let init_string = module.strings.get_or_intern("init");
        VM {
            module,
            frames: vec![],
            stack: gc::unique(vec![]),
            globals: gc::unique(HashMap::new()),
            upvalues: vec![],
            init_string,
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
        let native = gc::manage(NativeFunction { name, arity, code });

        self.globals
            .insert(name, Value::NativeFunction(native.as_gc()));
    }

    pub fn interpret(&mut self) -> Result<(), RuntimeError> {
        let function = gc::manage(Function {
            arity: 0,
            chunk_index: 0,
            name: self.module.get_or_intern(""),
        });

        let closure = gc::manage(Closure {
            function: function.as_gc(),
            upvalues: vec![],
        });

        self.frames.push(CallFrame {
            closure,
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

        if false {
            println!("instruction: {:?}", instruction);
            println!("stack:");
            for e in &*self.stack {
                println!("\t{:?}", e);
            }
        }

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
                    let concat = [&e1[..], &e2[..]].concat();
                    self.push(Value::Array(gc::manage(concat).as_gc()));
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
                        println!(
                            "undefined: {:?} / {:?}",
                            self.module.strings.resolve(*key),
                            key
                        );
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
            Instruction::GetSuper(index) => {
                let class = self.pop()?;
                if let Constant::String(key) = self.module.constants.get(index) {
                    if let Value::Class(superclass) = class {
                        if let Some(method) = superclass.borrow().methods.get(key) {
                            let bounded = BoundMethod {
                                receiver: gc::manage(RefCell::new(class)).as_gc(),
                                method: *method,
                            };
                            let constant = Value::BoundMethod(gc::manage(bounded).as_gc());
                            self.push(constant);
                        } else {
                            return Err(RuntimeError::UndefinedProperty);
                        }
                    } else {
                        println!("got instead: {:?}", class);
                        return Err(RuntimeError::ExpectedClass);
                    }
                } else {
                    return Err(RuntimeError::ExpectedStringConstant);
                }
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
                let callee = *self.peek_n(arity)?;
                self.call(callee, arity)?;
            }
            Instruction::Closure(index) => {
                if let Constant::Closure(closure) = self.module.get_constant(index) {
                    let mut upvalues: Vec<Gc<RefCell<Upvalue>>> = vec![];

                    for u in &closure.upvalues {
                        if u.is_local {
                            let frame = &self.frames[self.frames.len() - 1];
                            let base = frame.base_counter;
                            let index = base + u.slot;

                            let mut upvalue: Option<Gc<RefCell<Upvalue>>> = None;
                            for managed in self.upvalues.iter().rev() {
                                if (*managed).borrow().is_open(index) {
                                    upvalue = Some(managed.as_gc());
                                    break;
                                }
                            }

                            match upvalue {
                                Some(up) => upvalues.push(up),
                                None => {
                                    let managed = gc::manage(RefCell::new(Upvalue::Open(index)));
                                    self.upvalues.push(managed.clone());
                                    upvalues.push(managed.as_gc());
                                }
                            }
                        } else {
                            let frame = &self.frames[self.frames.len() - 1];
                            upvalues.push(frame.closure.upvalues[u.slot]);
                        }
                    }
                    let function = gc::manage(Function {
                        name: closure.function.name,
                        chunk_index: closure.function.chunk_index,
                        arity: closure.function.arity,
                    });

                    let closure = gc::manage(Closure {
                        function: function.as_gc(),
                        upvalues,
                    });

                    self.push(Value::Closure(closure.as_gc()));
                } else {
                    return Err(RuntimeError::ExpectedClosure);
                }
            }
            Instruction::Class(index) => {
                if let Constant::Class(class) = self.module.constants.get(index) {
                    let class = gc::manage(RefCell::new(Class {
                        name: class.name,
                        methods: HashMap::new(),
                    }));
                    self.push(Value::Class(class.as_gc()));
                } else {
                    return Err(RuntimeError::UnexpectedConstant);
                }
            }
            Instruction::Inherit => {
                if let Value::Class(superclass) = self.peek_n(1)? {
                    if let Value::Class(subclass) = self.peek()? {
                        let superclass_methods = superclass.borrow().methods.clone();
                        let mut subclass = subclass.borrow_mut();
                        subclass.methods.extend(
                            superclass_methods
                                .into_iter()
                                .map(|(k, v)| (k.clone(), v.clone())),
                        );
                    } else {
                        return Err(RuntimeError::ExpectedClass);
                    }
                } else {
                    return Err(RuntimeError::ExpectedClass);
                }
                self.pop()?;
            }
            Instruction::GetProperty(index) => {
                let instance_popped = self.pop()?;
                if let Value::Instance(instance) = instance_popped {
                    let instance = gc::root(instance);
                    if let Constant::String(property) = self.module.get_constant(index) {
                        if let Some(value) = instance.borrow().fields.get(property) {
                            self.push(*value);
                        } else if let Some(method) =
                            instance.borrow().class.borrow().methods.get(property)
                        {
                            let bounded = BoundMethod {
                                receiver: gc::manage(RefCell::new(instance_popped)).as_gc(),
                                method: *method,
                            };
                            let constant = Value::BoundMethod(gc::manage(bounded).as_gc());
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
                if let Value::Closure(closure) = self.peek()? {
                    if let Value::Class(class) = self.peek_n(1)? {
                        let name = closure.function.name;
                        class.borrow_mut().methods.insert(name, *closure);
                        self.pop()?;
                    } else {
                        return Err(RuntimeError::ExpectedClass);
                    }
                } else {
                    return Err(RuntimeError::ExpectedCallee);
                }
            }
            Instruction::SuperInvoke(method_sym, arity) => {
                self.super_invoke(method_sym, arity)?;
            }
            Instruction::Invoke(sym, arity) => {
                self.invoke(sym, arity)?;
            }
            Instruction::CloseUpvalue => {
                let index = self.stack.len() - 1;
                self.close_upvalues(index);
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
                self.push(Value::Array(gc::manage(elements).as_gc()))
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
                self.push(Value::Array(gc::manage(res).as_gc()));
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
                self.push(Value::Array(gc::manage(res).as_gc()));
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
                        } else if (index as usize) >= elements.len() {
                            return Err(RuntimeError::IndexOutOfBounds);
                        } else {
                            self.push(elements[index as usize].clone());
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
                println!();
            }
            Instruction::Return => {
                let res = self.pop()?;
                let frame = self
                    .frames
                    .pop()
                    .ok_or_else(|| RuntimeError::CallFrameEmpty)?;

                for i in frame.base_counter..self.stack.len() {
                    self.close_upvalues(i);
                }

                self.stack.truncate(frame.base_counter);

                if self.frames.is_empty() {
                    return Ok(InterpreterResult::Done);
                }
                self.push(res);
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
                    Ok(string.to_string())
                }
            }
            Value::Boolean(b) => Ok(format!("{}", b)),
            Value::None => Ok("none".to_string()),
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
            Value::Closure(closure) => Ok(format!(
                "closure: <{:?}>",
                self.module
                    .strings
                    .resolve(closure.function.name)
                    .expect("")
            )),
            Value::NativeFunction(native) => Ok(format!(
                "native_function: <{:?}>",
                self.module.strings.resolve(native.name).expect("")
            )),
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
            Value::Instance(instance) => Ok(format!(
                "instance_of: <{:?}>\n\t{:?}",
                self.module
                    .strings
                    .resolve((*(*instance).borrow().class).borrow().name)
                    .expect(""),
                instance.borrow().fields
            )),
            Value::BoundMethod(bounded) => Ok(format!(
                "bound_method: <{:?}>",
                self.module.strings.resolve(bounded.method.function.name)
            )),
        }
    }

    fn super_invoke(&mut self, name: Sym, arity: usize) -> Result<(), RuntimeError> {
        let receiver = *self.peek_n(arity)?;
        match receiver {
            Value::Class(class) => {
                if let Some(closure) = class.borrow().methods.get(&name) {
                    if closure.function.arity != arity {
                        return Err(RuntimeError::IncorrectArity);
                    }
                    self.begin_frame(*closure);
                } else {
                    return Err(RuntimeError::UndefinedProperty);
                }
            }
            _ => return Err(RuntimeError::ExpectedClass),
        };
        Ok(())
    }

    fn invoke(&mut self, name: Sym, arity: usize) -> Result<(), RuntimeError> {
        let receiver = *self.peek_n(arity)?;
        match receiver {
            Value::Instance(instance) => {
                if let Some(value) = instance.borrow().fields.get(&name) {
                    let len = self.stack.len();
                    self.stack[len - arity - 1] = *value;
                    self.call(*value, arity)?;
                } else if let Some(closure) = instance.borrow().class.borrow().methods.get(&name) {
                    if closure.function.arity != arity {
                        return Err(RuntimeError::IncorrectArity);
                    }
                    self.begin_frame(*closure);
                } else {
                    println!("for: {:?}", receiver);
                    println!("property: {:?}", self.module.strings.resolve(name));
                    return Err(RuntimeError::UndefinedProperty);
                }
            }
            _ => return Err(RuntimeError::ExpectedInstance),
        };
        Ok(())
    }

    fn call(&mut self, callee: Value, arity: usize) -> Result<(), RuntimeError> {
        match callee {
            Value::Closure(callee) => {
                if callee.function.arity != arity {
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
                let instance = gc::manage(RefCell::new(Instance {
                    class,
                    fields: HashMap::new(),
                }));

                let len = self.stack.len();
                self.stack[len - arity - 1] = Value::Instance(instance.as_gc());

                if let Some(method) = class.borrow().methods.get(&self.init_string) {
                    if method.function.arity != arity {
                        return Err(RuntimeError::IncorrectArity);
                    }
                    self.begin_frame(*method);
                } else if arity > 0 {
                    return Err(RuntimeError::IncorrectArity);
                }
            }
            Value::BoundMethod(callee) => {
                if callee.method.function.arity != arity {
                    return Err(RuntimeError::IncorrectArity);
                }

                let len = self.stack.len();
                self.stack[len - arity - 1] = *callee.receiver.borrow();
                self.begin_frame((*callee).method);
            }
            _ => return Err(RuntimeError::ExpectedCallee),
        }
        Ok(())
    }

    fn close_upvalues(&mut self, index: usize) {
        let value = self.stack[index];

        for upvalue in &self.upvalues {
            if upvalue.borrow().is_open(index) {
                upvalue.replace(Upvalue::Closed(value));
            }
        }

        self.upvalues.retain(|u| u.borrow().open());
    }

    fn begin_frame(&mut self, closure: Gc<Closure>) {
        let arity = closure.function.arity;
        let index = closure.function.chunk_index;
        self.frames.push(CallFrame {
            closure: gc::root(closure),
            ip: 0,
            base_counter: self.stack.len() - arity - 1,
            chunk_index: index,
        });
    }
}
