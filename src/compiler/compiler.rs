use crate::bytecode::chunk::Chunk;
use crate::bytecode::chunk::ConstantIndex;
use crate::bytecode::chunk::Instruction;
use crate::bytecode::chunk::StackIndex;
use crate::bytecode::constant::Constant;
use crate::bytecode::distance::Distance;
use crate::bytecode::module::Module;
use crate::compiler::class::Class;
use crate::compiler::error::CompileError;
use crate::compiler::function::Closure;
use crate::compiler::function::Function;
use crate::compiler::local::Locals;
use crate::compiler::local::Upvalue;
use crate::parser::ast::Expr;
use crate::parser::ast::Stmt;
use crate::parser::tokens::Symbol;
use crate::parser::tokens::Token;
use string_interner::StringInterner;
use string_interner::Sym;

#[derive(Debug, PartialEq)]
pub enum ContextType {
    Function,
    Method,
    Initializer,
    Script,
}

#[derive(Debug)]
pub struct CompilerContext {
    pub context_type: ContextType,
    pub enclosing: usize,
    pub no_enclosing: bool,
    pub chunk_index: usize,
    pub locals: Locals,
    pub upvalues: Vec<Upvalue>,
}

impl CompilerContext {
    pub fn new(
        context_type: ContextType,
        enclosing: usize,
        no_enclosing: bool,
        chunk_index: usize,
        strings: &mut StringInterner<Sym>,
    ) -> Self {
        let mut locals = Locals::new();

        match context_type {
            ContextType::Function => {
                let sym = strings.get_or_intern("");
                locals.insert(sym);
            }
            _ => {
                let sym = strings.get_or_intern("my");
                locals.insert(sym);
                locals.mark_initialized();
            }
        };

        CompilerContext {
            context_type,
            enclosing,
            no_enclosing,
            chunk_index,
            locals,
            upvalues: vec![],
        }
    }

    fn add_upvalue(&mut self, slot: usize, is_local: bool) -> StackIndex {
        for (index, upvalue) in self.upvalues.iter().enumerate() {
            if upvalue.slot == slot && upvalue.is_local == is_local {
                return index;
            }
        }
        self.upvalues.push(Upvalue { slot, is_local });

        self.upvalues.len() - 1
    }

    pub fn resolve_local(&self, sym: Sym) -> Result<Option<usize>, CompileError> {
        if let Some(local) = self.locals.get(sym) {
            if local.is_initialized {
                Ok(Some(local.slot))
            } else {
                Err(CompileError::VariableNotInitialized)
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub struct ClassContext {
    pub enclosing: usize,
    pub superclass: bool,
    pub name: Sym,
}

#[derive(Debug)]
pub struct Compiler {
    module: Module,
    contexts: Vec<CompilerContext>,
    classes: Vec<ClassContext>,
    debug: bool,
}

impl Compiler {
    pub fn new(strings: StringInterner<Sym>, debug: bool) -> Self {
        let mut strings = strings;
        let chunk_index = 0;

        let mut contexts: Vec<CompilerContext> = vec![];
        contexts.push(CompilerContext::new(
            ContextType::Script,
            0,
            true,
            chunk_index,
            &mut strings,
        ));

        let mut module = Module::new(strings);
        module.add_chunk();

        Compiler {
            module,
            contexts,
            classes: vec![],
            debug,
        }
    }

    fn current_class(&self) -> Option<&ClassContext> {
        self.classes.last()
    }

    fn current_context(&self) -> &CompilerContext {
        self.contexts
            .last()
            .expect("expected a &CompilerContext to exist")
    }

    fn current_context_mut(&mut self) -> &mut CompilerContext {
        self.contexts
            .last_mut()
            .expect("expected a &mut CompilerContext to exist")
    }

    fn current_chunk(&self) -> &Chunk {
        self.module.get_chunk(self.current_context().chunk_index)
    }

    fn current_chunk_mut(&mut self) -> &mut Chunk {
        self.module
            .get_chunk_mut(self.current_context().chunk_index)
    }

    fn add_instruction(&mut self, instruction: Instruction) -> usize {
        self.current_chunk_mut().add_instruction(instruction)
    }

    fn add_constant(&mut self, constant: Constant) -> ConstantIndex {
        self.module.add_constant(constant)
    }

    fn resolve_local(&mut self, sym: Sym) -> Result<Option<usize>, CompileError> {
        self.current_context_mut().resolve_local(sym)
    }

    fn resolve_upvalue(
        &mut self,
        context_index: usize,
        sym: Sym,
    ) -> Result<Option<usize>, CompileError> {
        let context = &self.contexts[context_index];
        let enclosing = context.enclosing;
        if !context.no_enclosing {
            if let Some(slot) = self.contexts[enclosing].resolve_local(sym)? {
                let enclosing_mut = &mut self.contexts[enclosing];
                enclosing_mut.locals.mark_captured(slot);
                let index = self.contexts[context_index].add_upvalue(slot, true);
                return Ok(Some(index));
            } else if let Some(slot) = self.resolve_upvalue(enclosing, sym)? {
                let index = self.contexts[context_index].add_upvalue(slot, false);
                return Ok(Some(index));
            }
        }
        Ok(None)
    }

    fn begin_scope(&mut self) {
        self.current_context_mut().locals.enter_scope();
    }

    fn end_scope(&mut self) {
        let locals = self.current_context_mut().locals.leave_scope();
        for (k, v) in self.module.strings.iter() {
            println!("{:?}: {:?}", k ,v);
        }
        println!("Locals: {:?}", locals);
        println!("cur instr: {:?}",  self.current_chunk().instructions);
        for local in locals.iter().rev() {
            if local.is_captured {
                self.add_instruction(Instruction::CloseUpvalue);
            } else {
                self.add_instruction(Instruction::Pop);
            }
        }
    }

    pub fn compile(&mut self, statements: &[Stmt]) -> Result<&Module, CompileError> {
        self.compile_program(statements)?;

        if self.debug {
            for chunk in &self.module.chunks {
                chunk.disassemble(&self.module.constants);
                println!();
            }
        }

        Ok(&self.module)
    }

    fn compile_program(&mut self, statements: &[Stmt]) -> Result<(), CompileError> {
        for statement in statements {
            self.compile_statement(statement)?;
        }
        self.add_instruction(Instruction::None);
        self.add_instruction(Instruction::Return);
        Ok(())
    }

    fn compile_statement(&mut self, statement: &Stmt) -> Result<(), CompileError> {
        match statement {
            Stmt::Expression(ref expr) => self.compile_expression_statement(expr),
            Stmt::Print(ref expr_list) => self.compile_print(expr_list),
            Stmt::Return(ref expr) => self.compile_return(expr),
            Stmt::Assign(ref sym, ref expr) => self.compile_assign(*sym, expr),
            Stmt::Block(ref statements) => self.compile_block(statements),
            Stmt::If(ref condition, ref body, ref next) => self.compile_if(condition, body, next),
            Stmt::While(ref condition, ref body) => self.compile_while(condition, body),
            Stmt::Function(ref sym, ref params, ref body) => {
                self.compile_function(*sym, params, body)
            }
            Stmt::Class(ref class_name, ref super_name, ref methods) => {
                self.compile_class(*class_name, super_name, methods)
            }
        }?;

        Ok(())
    }

    fn compile_expression_statement(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match expr {
            Expr::Assign(_, _) => {
                self.compile_expression(expr)?;
            }
            _ => {
                self.compile_expression(expr)?;
                self.add_instruction(Instruction::Pop);
            }
        }
        Ok(())
    }

    fn compile_expression(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match expr {
            Expr::Number(ref distance) => self.compile_number(distance),
            Expr::String(ref sym) => self.compile_string(*sym),
            Expr::Boolean(ref boolean) => self.compile_boolean(*boolean),
            Expr::None => self.compile_none(),
            Expr::Variable(ref sym) => self.compile_variable(*sym),
            Expr::Assign(ref sym, ref right) => self.compile_assign(*sym, right),
            Expr::List(ref elements) => self.compile_list(elements),
            Expr::Index(ref sym, ref expr) => self.compile_index(sym, expr),
            Expr::Slice(ref start, ref stop, ref step) => self.compile_slice(start, stop, step),
            Expr::Range(ref start, ref stop, ref step) => self.compile_range(start, stop, step),
            Expr::Binary(ref left, ref op, ref right) => self.compile_binary(left, op, right),
            Expr::Logical(ref left, ref op, ref right) => self.compile_logical(left, op, right),
            Expr::Grouping(ref expr) => self.compile_grouping(expr),
            Expr::Pair(ref left, ref right) => self.compile_pair(left, right),
            Expr::Unary(ref op, ref right) => self.compile_unary(op, right),
            Expr::Call(ref callee, ref arguments) => self.compile_call(callee, arguments),
            Expr::Get(ref left, ref sym) => self.compile_get(left, *sym),
            Expr::Set(ref left, ref sym, ref right) => self.compile_set(left, *sym, right),
            Expr::My(ref sym) => self.compile_my(*sym),
            Expr::Super(ref sym, ref name) => self.compile_super(*sym, *name),
            Expr::Invoke(ref left, ref sym, ref args) => self.compile_invoke(left, *sym, args),
            Expr::SuperInvoke(ref sym, ref name, ref args) => {
                self.compile_super_invoke(*sym, *name, args)
            }
        }?;

        Ok(())
    }

    fn compile_number(&mut self, distance: &Distance) -> Result<(), CompileError> {
        let constant = self.add_constant(Constant::Number(*distance));
        self.add_instruction(Instruction::Constant(constant));
        Ok(())
    }

    fn compile_string(&mut self, sym: Sym) -> Result<(), CompileError> {
        let constant = self.add_constant(Constant::String(sym));
        self.add_instruction(Instruction::Constant(constant));
        Ok(())
    }

    fn compile_boolean(&mut self, boolean: bool) -> Result<(), CompileError> {
        if boolean {
            self.add_instruction(Instruction::True);
        } else {
            self.add_instruction(Instruction::False);
        }
        Ok(())
    }

    fn compile_none(&mut self) -> Result<(), CompileError> {
        self.add_instruction(Instruction::None);
        Ok(())
    }

    fn compile_variable(&mut self, sym: Sym) -> Result<(), CompileError> {
        if let Some(local) = self.resolve_local(sym)? {
            self.add_instruction(Instruction::GetLocal(local));
        } else if let Some(upvalue) = self.resolve_upvalue(self.contexts.len() - 1, sym)? {
            self.add_instruction(Instruction::GetUpvalue(upvalue));
        } else {
            let constant = self.add_constant(Constant::String(sym));
            self.add_instruction(Instruction::GetGlobal(constant));
        }
        Ok(())
    }

    fn compile_list(&mut self, expressions: &[Expr]) -> Result<(), CompileError> {
        for expression in expressions {
            self.compile_expression(expression)?;
        }

        self.add_instruction(Instruction::List(expressions.len()));

        Ok(())
    }

    fn compile_index(&mut self, left: &Expr, right: &Expr) -> Result<(), CompileError> {
        match left {
            Expr::Variable(_) => self.compile_expression(left)?,
            Expr::Slice(_, _, _) => {
                self.compile_expression(left)?;
            }
            _ => {
                self.compile_expression(left)?;
                self.add_instruction(Instruction::Index);
            }
        };

        match right {
            Expr::Pair(a, b) => {
                match **a {
                    Expr::Number(_) | Expr::Variable(_) => {
                        self.compile_expression(a)?;
                        self.add_instruction(Instruction::Index);
                    }
                    Expr::None => (),
                    _ => self.compile_expression(a)?,
                };
                match **b {
                    Expr::Number(_) | Expr::Variable(_) => {
                        self.compile_expression(b)?;
                        self.add_instruction(Instruction::Index);
                    }
                    Expr::None => (),
                    _ => self.compile_expression(b)?,
                };
            }
            _ => return Err(CompileError::UnexpectedExpression),
        };

        Ok(())
    }

    fn compile_slice(
        &mut self,
        start: &Option<Box<Expr>>,
        stop: &Option<Box<Expr>>,
        step: &Option<Box<Expr>>,
    ) -> Result<(), CompileError> {
        match start {
            Some(expr) => self.compile_expression(expr)?,
            _ => self.compile_none()?,
        };

        match stop {
            Some(expr) => self.compile_expression(expr)?,
            _ => self.compile_none()?,
        };

        match step {
            Some(expr) => self.compile_expression(expr)?,
            _ => self.compile_none()?,
        };
        self.add_instruction(Instruction::Slice);
        Ok(())
    }

    fn compile_range(
        &mut self,
        start: &Expr,
        stop: &Expr,
        step: &Option<Box<Expr>>,
    ) -> Result<(), CompileError> {
        self.compile_expression(start)?;
        self.compile_expression(stop)?;

        match step {
            Some(expr) => self.compile_expression(expr)?,
            _ => self.compile_none()?,
        };
        self.add_instruction(Instruction::Range);
        Ok(())
    }

    fn compile_binary(
        &mut self,
        left: &Expr,
        op: &Token,
        right: &Expr,
    ) -> Result<(), CompileError> {
        self.compile_expression(left)?;
        self.compile_expression(right)?;

        let sym = &op.symbol;

        match sym {
            Symbol::Plus => self.add_instruction(Instruction::Add),
            Symbol::Minus => self.add_instruction(Instruction::Subtract),
            Symbol::Star => self.add_instruction(Instruction::Multiply),
            Symbol::Slash => self.add_instruction(Instruction::Divide),
            Symbol::Modulo => self.add_instruction(Instruction::Modulo),
            Symbol::Carat => self.add_instruction(Instruction::Pow),
            Symbol::Less => self.add_instruction(Instruction::Less),
            Symbol::LessEqual => self.add_instruction(Instruction::LessEqual),
            Symbol::Greater => self.add_instruction(Instruction::Greater),
            Symbol::GreaterEqual => self.add_instruction(Instruction::GreaterEqual),
            Symbol::NotEqual => self.add_instruction(Instruction::NotEqual),
            Symbol::EqualEqual => self.add_instruction(Instruction::EqualEqual),
            _ => unreachable!(),
        };

        Ok(())
    }

    fn compile_logical(
        &mut self,
        left: &Expr,
        op: &Token,
        right: &Expr,
    ) -> Result<(), CompileError> {
        match op.symbol {
            Symbol::And => self.compile_and(left, right),
            Symbol::Or => self.compile_or(left, right),
            _ => unreachable!(),
        }
    }

    fn compile_and(&mut self, left: &Expr, right: &Expr) -> Result<(), CompileError> {
        self.compile_expression(left)?;
        let next_jump = self.add_instruction(Instruction::JumpIfFalse(0));
        self.add_instruction(Instruction::Pop);
        self.compile_expression(right)?;
        self.current_chunk_mut().patch_instruction(next_jump);
        Ok(())
    }

    fn compile_or(&mut self, left: &Expr, right: &Expr) -> Result<(), CompileError> {
        self.compile_expression(left)?;
        let next_jump = self.add_instruction(Instruction::JumpIfTrue(0));
        self.add_instruction(Instruction::Pop);
        self.compile_expression(right)?;
        self.current_chunk_mut().patch_instruction(next_jump);
        Ok(())
    }

    fn compile_grouping(&mut self, expr: &Expr) -> Result<(), CompileError> {
        self.compile_expression(expr)?;
        Ok(())
    }

    fn compile_pair(&mut self, left: &Expr, right: &Expr) -> Result<(), CompileError> {
        match left {
            Expr::Number(_) => {
                self.compile_expression(left)?;
                self.add_instruction(Instruction::Index);
            }
            Expr::None => (),
            _ => self.compile_expression(left)?,
        };

        match right {
            Expr::Number(_) => {
                self.compile_expression(right)?;
                self.add_instruction(Instruction::Index);
            }
            Expr::None => (),
            _ => self.compile_expression(right)?,
        };
        Ok(())
    }

    fn compile_unary(&mut self, op: &Token, right: &Expr) -> Result<(), CompileError> {
        self.compile_expression(right)?;
        match op.symbol {
            Symbol::Not => self.add_instruction(Instruction::Not),
            Symbol::Minus => self.add_instruction(Instruction::Negate),
            _ => unreachable!(),
        };
        Ok(())
    }

    fn compile_call(&mut self, callee: &Expr, arguments: &[Expr]) -> Result<(), CompileError> {
        self.compile_expression(callee)?;
        for arg in arguments {
            self.compile_expression(arg)?;
        }
        self.add_instruction(Instruction::Call(arguments.len()));
        Ok(())
    }

    fn compile_get(&mut self, left: &Expr, sym: Sym) -> Result<(), CompileError> {
        self.compile_expression(left)?;
        let constant = self.add_constant(Constant::String(sym));
        self.add_instruction(Instruction::GetProperty(constant));
        Ok(())
    }

    fn compile_set(&mut self, left: &Expr, sym: Sym, right: &Expr) -> Result<(), CompileError> {
        self.compile_expression(left)?;
        self.compile_expression(right)?;

        let constant = self.add_constant(Constant::String(sym));
        self.add_instruction(Instruction::SetProperty(constant));

        Ok(())
    }

    fn compile_my(&mut self, sym: Sym) -> Result<(), CompileError> {
        match self.current_class() {
            None => return Err(CompileError::MyUsedOutsideClass),
            Some(_) => (),
        };

        self.compile_variable(sym)?;
        Ok(())
    }

    fn compile_super(&mut self, sym: Sym, name: Sym) -> Result<(), CompileError> {
        match self.current_class() {
            None => return Err(CompileError::SuperUsedOutsideClass),
            Some(class_context) => {
                if !class_context.superclass {
                    return Err(CompileError::SuperUsedWithoutSuperclass);
                }
            }
        };

        let my_sym = self.module.strings.get_or_intern("my");

        self.compile_variable(my_sym)?;
        self.compile_variable(sym)?;

        let constant = self.add_constant(Constant::String(name));
        self.add_instruction(Instruction::GetSuper(constant));

        Ok(())
    }

    fn compile_invoke(&mut self, left: &Expr, sym: Sym, args: &[Expr]) -> Result<(), CompileError> {
        self.compile_expression(left)?;
        for arg in args {
            self.compile_expression(arg)?;
        }
        self.add_instruction(Instruction::Invoke(sym, args.len()));
        Ok(())
    }

    fn compile_super_invoke(
        &mut self,
        super_sym: Sym,
        method_sym: Sym,
        args: &[Expr],
    ) -> Result<(), CompileError> {
        let my_sym = self.module.strings.get_or_intern("my");
        self.compile_variable(my_sym)?;
        self.compile_variable(super_sym)?;

        for arg in args {
            self.compile_expression(arg)?;
        }
        self.add_instruction(Instruction::SuperInvoke(method_sym, args.len()));
        Ok(())
    }

    fn compile_print(&mut self, expr_list: &[Expr]) -> Result<(), CompileError> {
        for expr in expr_list {
            self.compile_expression(expr)?;
        }
        self.add_instruction(Instruction::Print(expr_list.len()));
        Ok(())
    }

    fn compile_return(&mut self, expr: &Option<Box<Expr>>) -> Result<(), CompileError> {
        if self.current_context().context_type == ContextType::Script {
            Err(CompileError::ReturnInScript)
        } else if self.current_context().context_type == ContextType::Initializer {
            Err(CompileError::ReturnInInitializer)
        } else {
            if let Some(expr) = expr {
                self.compile_expression(expr)?;
            } else {
                self.compile_none()?;
            }
            self.add_instruction(Instruction::Return);
            Ok(())
        }
    }

    fn declare_variable(&mut self, sym: Sym) {
        if self.current_context().locals.depth > 0 {
            self.current_context_mut().locals.insert(sym);
        }
    }

    fn define_variable(&mut self, sym: Sym) {
        if self.current_context().locals.depth > 0 {
            self.current_context_mut().locals.mark_initialized();
        } else {
            let constant = self.add_constant(Constant::String(sym));
            self.add_instruction(Instruction::SetGlobal(constant));
        }
    }

    fn compile_assign(&mut self, sym: Sym, expr: &Expr) -> Result<(), CompileError> {
        self.declare_variable(sym);
        self.compile_expression(expr)?;
        self.define_variable(sym);

        if let Some(local) = self.resolve_local(sym)? {
            self.add_instruction(Instruction::SetLocal(local));
        } else if let Some(upvalue) = self.resolve_upvalue(self.contexts.len() - 1, sym)? {
            self.add_instruction(Instruction::SetUpvalue(upvalue));
        }
        Ok(())
    }

    fn compile_block(&mut self, statements: &[Stmt]) -> Result<(), CompileError> {
        for statement in statements {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    fn compile_if(
        &mut self,
        condition: &Expr,
        body: &Stmt,
        next: &Option<Box<Stmt>>,
    ) -> Result<(), CompileError> {
        self.compile_expression(condition)?;
        let next_index = self.add_instruction(Instruction::JumpIfFalse(0));
        self.add_instruction(Instruction::Pop);
        self.compile_statement(body)?;

        if let Some(statement) = next {
            let index = self.add_instruction(Instruction::Jump(0));
            self.current_chunk_mut().patch_instruction(next_index);
            self.add_instruction(Instruction::Pop);
            self.compile_statement(statement)?;
            self.current_chunk_mut().patch_instruction(index);
        } else {
            self.current_chunk_mut().patch_instruction(next_index);
        }

        Ok(())
    }

    fn compile_while(&mut self, condition: &Expr, body: &Stmt) -> Result<(), CompileError> {
        let start_jump = self.current_chunk().instructions.len();
        self.compile_expression(condition)?;

        let exit_jump = self.add_instruction(Instruction::JumpIfFalse(0));
        self.add_instruction(Instruction::Pop);

        self.compile_statement(body)?;

        let loop_jump = self.add_instruction(Instruction::Jump(0));
        self.current_chunk_mut()
            .patch_instruction_to(loop_jump, start_jump);
        self.current_chunk_mut().patch_instruction(exit_jump);
        self.add_instruction(Instruction::Pop);

        Ok(())
    }

    fn compile_function(
        &mut self,
        sym: Sym,
        params: &[Sym],
        body: &[Stmt],
    ) -> Result<(), CompileError> {
        self.declare_variable(sym);
        if self.current_context().locals.depth > 0 {
            self.current_context_mut().locals.mark_initialized();
        }

        let chunk_index = self.module.add_chunk();
        let enclosing = self.contexts.len() - 1;
        self.contexts.push(CompilerContext::new(
            ContextType::Function,
            enclosing,
            false,
            chunk_index,
            &mut self.module.strings,
        ));

        self.begin_scope();

        for param in params {
            self.declare_variable(*param);
            self.define_variable(*param);
        }

        self.compile_block(body)?;

        match body.last() {
            Some(Stmt::Return(_)) => (),
            _ => {
                if self.current_context().context_type == ContextType::Initializer {
                    self.add_instruction(Instruction::GetLocal(0));
                } else {
                    self.add_instruction(Instruction::None);
                }
                self.add_instruction(Instruction::Return);
            }
        };


        let context = self
            .contexts
            .pop()
            .expect("Expect a context during function compilation");

        // self.current_context_mut().locals.depth -= 1 ;

        let function = Function {
            name: sym,
            chunk_index,
            arity: params.len(),
        };

        let closure = Closure {
            function,
            upvalues: context.upvalues,
        };

        let constant = self.add_constant(Constant::Closure(closure));
        self.add_instruction(Instruction::Closure(constant));

        self.define_variable(sym);
        // self.current_context_mut().locals.depth -= 1 ;
        Ok(())
    }

    fn compile_method(
        &mut self,
        sym: Sym,
        params: &[Sym],
        body: &[Stmt],
    ) -> Result<(), CompileError> {

        self.begin_scope();

        self.declare_variable(sym);
        if self.current_context().locals.depth > 0 {
            self.current_context_mut().locals.mark_initialized();
        }

        let chunk_index = self.module.add_chunk();
        let enclosing = self.contexts.len() - 1;

        let method_type = if self.module.strings.get_or_intern("init") == sym {
            ContextType::Initializer
        } else {
            ContextType::Method
        };

        self.contexts.push(CompilerContext::new(
            method_type,
            enclosing,
            false,
            chunk_index,
            &mut self.module.strings,
        ));

        for param in params {
            self.declare_variable(*param);
            self.define_variable(*param);
        }

        self.compile_block(body)?;

        match body.last() {
            Some(Stmt::Return(_)) => (),
            _ => {
                if self.current_context().context_type == ContextType::Initializer {
                    self.add_instruction(Instruction::GetLocal(0));
                } else {
                    self.add_instruction(Instruction::None);
                }
                self.add_instruction(Instruction::Return);
            }
        };

        let context = self
            .contexts
            .pop()
            .expect("Expect a context during function compilation");

        self.current_context_mut().locals.depth -= 1 ;

        let function = Function {
            name: sym,
            chunk_index,
            arity: params.len(),
        };

        let closure = Closure {
            function,
            upvalues: context.upvalues,
        };

        let constant = self.add_constant(Constant::Closure(closure));
        self.add_instruction(Instruction::Closure(constant));

        // self.define_variable(sym);
        Ok(())
    }

    fn compile_class(
        &mut self,
        class_name: Sym,
        super_option: &Option<Sym>,
        methods: &[Stmt],
    ) -> Result<(), CompileError> {
        self.classes.push(ClassContext {
            enclosing: self.classes.len(),
            superclass: super_option.is_some(),
            name: class_name,
        });

        self.declare_variable(class_name);
        let constant = self.add_constant(Constant::Class(Class { name: class_name }));
        self.add_instruction(Instruction::Class(constant));
        self.define_variable(class_name);

        if let Some(super_name) = super_option {
            if class_name == *super_name {
                return Err(CompileError::ClassCannotSuperItself);
            } else {
                let super_sym = self.module.strings.get_or_intern("super");
                self.current_context_mut().locals.insert(super_sym);
                self.current_context_mut().locals.mark_initialized();
                self.compile_variable(*super_name)?;

                self.begin_scope();

                self.compile_variable(*super_name)?;
                self.compile_variable(class_name)?;

                self.add_instruction(Instruction::Inherit);
            }
        }

        self.compile_variable(class_name)?;

        // self.begin_scope();
        for method in methods {
            if let Stmt::Function(ref sym, ref params, ref body) = method {
                self.compile_method(*sym, params, body)?;
                self.add_instruction(Instruction::Method);
            }
        }

        // self.end_scope();

        // self.add_instruction(Instruction::Pop);

        if let Some(class) = self.current_class() {
            if class.superclass {
                self.end_scope();
            }
        }

        self.add_instruction(Instruction::Pop);


        self.classes.pop().expect("Expect class to pop");

        Ok(())
    }
}
