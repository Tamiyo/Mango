use crate::chunk::{Chunk, Instruction, ConstantIndex, ChunkIndex, StackIndex, InstructionIndex};
use crate::constant::Constant;
use crate::ast::{Stmt, Expr, Identifier};
use crate::error::CompileError;
use crate::token::Symbol;
use crate::module::Module;
use crate::local::{Local, Locals};

// https://www.craftinginterpreters.com/local-variables.html
#[derive(Debug, PartialEq)]
pub enum ContextType {
    Function,
    // Initializer,
    // Method,
    Script,
}

struct CompilerContext {
    context_type: ContextType,
    chunk_index: ChunkIndex,

    locals: Locals,
}

impl CompilerContext {
    pub fn new(context_type: ContextType, chunk_index: ChunkIndex) -> Self {
        CompilerContext {
            context_type,
            chunk_index,
            locals: Locals::new(),
        }
    }

    pub fn scope_depth(&self) -> usize {
        self.locals.depth()
    }

    pub fn resolve_local(&self, name: &str) -> Result<Option<StackIndex>, CompileError> {
        if let Some(local) = self.locals.get(name) {
            if local.initialized() {
                Ok(Some(local.location()))
            } else {
                Err(CompileError::from(format!("cannot assign to undefined variable '{}'", name)))
            }
        } else {
            Ok(None)
        }
    }
}

pub struct Compiler {
    module: Module,
    contexts: Vec<CompilerContext>,
}

impl Compiler {
    pub fn new() -> Self {
        let mut module = Module::new();
        let index = module.add_chunk();

        let mut contexts: Vec<CompilerContext> = vec![];
        contexts.push(CompilerContext::new(ContextType::Script, index));

        Compiler {
            module,
            contexts,
        }
    }

    pub fn compile(&mut self, stmts: &[Stmt]) -> Result<&Module, CompileError> {
        self.program(stmts)?;
        self.disassemble();
        Ok(&self.module)
    }

    // module
    fn current_chunk(&self) -> &Chunk {
        self.module.chunk(self.current_context().chunk_index)
    }

    fn current_chunk_mut(&mut self) -> &mut Chunk {
        self.module.chunk_mut(self.current_context().chunk_index)
    }

    fn patch_instruction(&mut self, index: InstructionIndex) {
        self.current_chunk_mut().patch_instruction(index);
    }

    fn patch_instruction_to(&mut self, index: InstructionIndex, to: InstructionIndex) {
        self.current_chunk_mut().patch_instruction_to(index, to);
    }

    fn instruction_index(&self) -> InstructionIndex {
        self.current_chunk().instructions.len()
    }

    fn add_instruction(&mut self, instruction: Instruction) -> InstructionIndex {
        let context = self.contexts.last().expect("expected some compiler context");
        let index = context.chunk_index;
        self.module.add_instruction(index, instruction)
    }

    fn add_constant<C: Into<Constant>>(&mut self, constant: C) -> ConstantIndex {
        self.module.add_constant(constant.into())
    }

    pub fn disassemble(&self) {
        println!("chunk constants: {:?}", self.module.constants());
        for chunk in self.module.chunks() {
            chunk.disassemble(self.module.constants());
            println!();
        }
    }

    // context
    fn new_context(&mut self, context_type: ContextType) {
        let chunk_index = self.module.add_chunk();
        self.contexts.push(CompilerContext::new(
            context_type,
            chunk_index));
    }

    fn current_context(&self) -> &CompilerContext {
        self.contexts.last().expect("expected a compiler context to exist")
    }

    fn current_context_mut(&mut self) -> &mut CompilerContext {
        self.contexts.last_mut().expect("expected a compiler context to exist")
    }

    // scope
    fn begin_scope(&mut self) {
        self.current_context_mut().locals.begin_scope();
    }

    fn end_scope(&mut self) {
        self.current_context_mut().locals.end_scope();
    }

    fn local_depth(&mut self) -> bool {
        self.current_context().locals.depth() != 0
    }

    fn find_local_at_depth(&mut self, name: &str) -> Option<&Local> {
        self.current_context().locals.get_at_depth(name)
    }

    fn mark_initialized(&mut self) {
        self.current_context_mut().locals.mark_initialized();
    }

    fn resolve_local(&self, name: &str) -> Result<Option<StackIndex>, CompileError> {
        self.current_context().resolve_local(name)
    }

    // variable
    fn declare_variable(&mut self, name: &str) {
        if self.local_depth() && self.find_local_at_depth(name) == None {
            self.current_context_mut().locals.insert(name);
        }
    }

    fn define_variable(&mut self, name: &str) {
        if self.local_depth() {
            self.mark_initialized();
        } else {
            let constant = self.add_constant(name);
            self.add_instruction(Instruction::SetGlobal(constant));
        }
    }

    // statement
    fn program(&mut self, stmts: &[Stmt]) -> Result<(), CompileError> {
        for stmt in stmts {
            match self.statement(stmt) {
                Ok(()) => {}
                Err(error) => {
                    panic!(format!("CompileError: {}", error.error));
                }
            }
        }
        self.add_instruction(Instruction::Return);
        Ok(())
    }

    fn statement(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        match stmt {
            Stmt::Assign(ref name, ref expr) => self.assign_statement(name, expr),
            Stmt::Block(ref stmts) => self.block_statement(stmts),
            Stmt::Print(ref expr) => self.print_statement(expr),
            Stmt::If(ref condition, ref block, ref next) => self.if_statement(condition, block, next),
            Stmt::While(ref condition, ref block) => self.while_statement(condition, block),
            Stmt::Function(ref name, ref params, ref body) => self.function(name, params, body),
            Stmt::Return(ref val) => self.return_statement(val),
            Stmt::Expression(ref expr) => self.expression(expr),
            _ => Err(CompileError::from(format!("Unrecognized statement {:?}", stmt)))
        }
    }

    fn assign_statement(&mut self, name: &str, expr: &Expr) -> Result<(), CompileError> {
        self.expression(expr)?;

        if self.current_context().scope_depth() > 0 {
            let local = match self.resolve_local(name)? {
                Some(local) => local,
                None => self.current_context_mut().locals.insert(name)
            };
            self.current_context_mut().locals.mark_initialized();
            self.add_instruction(Instruction::SetLocal(local));
        } else {
            let constant = self.add_constant(name);
            self.add_instruction(Instruction::SetGlobal(constant));
        }
        Ok(())
    }

    fn block_statement(&mut self, stmts: &[Stmt]) -> Result<(), CompileError> {
        self.begin_scope();
        for stmt in stmts {
            self.statement(stmt)?;
        }
        self.end_scope();
        Ok(())
    }

    fn print_statement(&mut self, expr: &Expr) -> Result<(), CompileError> {
        self.expression(expr)?;
        self.add_instruction(Instruction::Print);
        Ok(())
    }

    fn if_statement(&mut self, condition: &Expr, block: &Stmt, next: &Option<Box<Stmt>>) -> Result<(), CompileError> {
        self.expression(condition)?;

        let next_index = self.add_instruction(Instruction::JumpIfFalse(0));
        self.add_instruction(Instruction::Pop);
        self.statement(block)?;

        if let Some(stmt) = next {
            let index = self.add_instruction(Instruction::Jump(0));
            self.patch_instruction(next_index);
            self.add_instruction(Instruction::Pop);
            self.statement(stmt.as_ref())?;
            self.patch_instruction(index);
        } else {
            self.patch_instruction(next_index);
        }
        Ok(())
    }

    /// TODO - let buf = "x = 0; while (x < 4) { x = x + 1; }"; fails
    /// thread 'main' panicked at 'CompileError: undefined variable 'x'', src\compiler.rs:178:21
    fn while_statement(&mut self, condition: &Expr, block: &Stmt) -> Result<(), CompileError> {
        let start_jump = self.instruction_index();
        self.expression(condition)?;

        let exit_jump = self.add_instruction(Instruction::JumpIfFalse(0));
        self.add_instruction(Instruction::Pop);
        self.statement(block)?;

        let loop_jump = self.add_instruction(Instruction::Jump(0));
        self.patch_instruction_to(loop_jump, start_jump);
        self.patch_instruction(exit_jump);
        self.add_instruction(Instruction::Pop);

        Ok(())
    }

    fn function(&mut self, name: &str, params: &[Identifier], body: &[Stmt]) -> Result<(), CompileError> {
        self.declare_variable(name);
        if self.local_depth() {
            self.mark_initialized();
        }

        self.new_context(ContextType::Function);
        self.begin_scope();

        for param in params {
            self.declare_variable(param);
            self.define_variable(param);
        }

        self.block_statement(body)?;

        match body.last() {
            Some(Stmt::Return(_)) => (),
            _ => {
                self.add_instruction(Instruction::None);
                self.add_instruction(Instruction::Return);
            }
        };
        self.contexts.pop();
        Ok(())
    }

    fn return_statement(&mut self, expr: &Option<Box<Expr>>) -> Result<(), CompileError> {
        if self.current_context().context_type == ContextType::Script {
            Err(CompileError::from("Cannot return from top-level context"))
        } else {
            if let Some(expr) = expr {
                self.expression(expr.as_ref())?;
            } else {
                self.none()?;
            }
            self.add_instruction(Instruction::Return);
            Ok(())
        }
    }

    // expression
    fn expression(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match *expr {
            Expr::Number(num) => self.number(num),
            Expr::String(ref string) => self.string(string),
            Expr::Boolean(boolean) => self.boolean(boolean),

            Expr::Variable(ref string) => self.variable(string),
            Expr::None => self.none(),

            Expr::Binary(ref left, ref op, ref right) => self.binary(left, op, right),
            Expr::Logical(ref left, ref op, ref right) => self.logical(left, op, right),
            Expr::Grouping(ref expr) => self.expression(expr),
            Expr::Unary(ref op, ref expr) => self.unary(op, expr),

            Expr::Call(ref left, ref args) => self.call(left, args),

            _ => Err(CompileError::from("Unexpected expression!"))
        }
    }

    fn binary(&mut self, left: &Expr, op: &Symbol, right: &Expr) -> Result<(), CompileError> {
        self.expression(left)?;
        self.expression(right)?;

        match *op {
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
            _ => panic!("Unknown Binary Operator")
        };

        Ok(())
    }

    fn logical(&mut self, left: &Expr, op: &Symbol, right: &Expr) -> Result<(), CompileError> {
        match op {
            Symbol::And => self.and(left, right),
            Symbol::Or => self.or(left, right),
            _ => Err(CompileError::from(format!("Expected logical op, got {:?} instead", op)))
        }
    }

    fn and(&mut self, left: &Expr, right: &Expr) -> Result<(), CompileError> {
        self.expression(left)?;
        let next_jump = self.add_instruction(Instruction::JumpIfFalse(0));
        self.add_instruction(Instruction::Pop);
        self.expression(right)?;
        self.patch_instruction(next_jump);
        Ok(())
    }

    fn or(&mut self, left: &Expr, right: &Expr) -> Result<(), CompileError> {
        self.expression(left)?;
        let next_jump = self.add_instruction(Instruction::JumpIfTrue(0));
        self.add_instruction(Instruction::Pop);
        self.expression(right)?;
        self.patch_instruction(next_jump);
        Ok(())
    }

    fn unary(&mut self, op: &Symbol, expr: &Expr) -> Result<(), CompileError> {
        self.expression(expr)?;
        match *op {
            Symbol::Not => self.add_instruction(Instruction::Not),
            _ => panic!("Unknown unary Operator")
        };
        Ok(())
    }

    /// TODO https://www.craftinginterpreters.com/global-variables.html
    /// The compiler adds a global variable’s name to the constant table as a string every time an identifier is encountered.
    /// It creates a new constant each time, even if that variable name is already in a previous slot in the constant table.
    /// That’s wasteful in cases where the same variable is referenced multiple times by the same function.
    /// That in turn increases the odds of filling up the constant table and running out of slots, since we only allow 256 constants in a single chunk.
    ///
    /// Optimize this. How does your optimization affect the performance of the compiler compared to the runtime? Is this the right trade-off?
    fn variable(&mut self, name: &str) -> Result<(), CompileError> {
        if self.current_context().scope_depth() > 0 {
            if let Some(local) = self.resolve_local(name)? {
                self.add_instruction(Instruction::GetLocal(local));
            } else {
                return Err(CompileError::from(format!("undefined variable '{}'", name)));
            }
        } else {
            let constant = self.add_constant(name);
            self.add_instruction(Instruction::GetGlobal(constant));
        }
        Ok(())
    }

    fn none(&mut self) -> Result<(), CompileError> {
        self.add_instruction(Instruction::None);
        Ok(())
    }

    fn number(&mut self, num: f64) -> Result<(), CompileError> {
        let constant = self.add_constant(num);
        self.add_instruction(Instruction::Constant(constant));
        Ok(())
    }

    fn string(&mut self, string: &str) -> Result<(), CompileError> {
        let constant = self.add_constant(string);
        self.add_instruction(Instruction::Constant(constant));
        Ok(())
    }

    fn boolean(&mut self, boolean: bool) -> Result<(), CompileError> {
        if boolean {
            self.add_instruction(Instruction::True);
        } else {
            self.add_instruction(Instruction::False);
        }
        Ok(())
    }

    fn call(&mut self, left: &Expr, args: &[Expr]) -> Result<(), CompileError> {
        self.expression(left)?;
        for arg in args {
            self.expression(arg)?;
        }
        self.add_instruction(Instruction::Call(args.len()));
        Ok(())
    }
}