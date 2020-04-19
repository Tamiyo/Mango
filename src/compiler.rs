use crate::chunk::{Chunk, Instruction, ConstantIndex, ChunkIndex, StackIndex, InstructionIndex};
use crate::constant::Constant;
use crate::ast::{Stmt, Expr};
use crate::error::CompileError;
use crate::token::Symbol;
use crate::module::Module;
use crate::local::{Local, Locals};

// https://www.craftinginterpreters.com/local-variables.html
pub enum ContextType {
    Function,
    Initializer,
    Method,
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

    // chunk
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

    fn add_instruction(&mut self, instruction: Instruction) -> InstructionIndex {
        let context = self.contexts.last().expect("expected some compiler context");
        let index = context.chunk_index;
        self.module.add_instruction(index, instruction)
    }

    fn add_constant<C: Into<Constant>>(&mut self, constant: C) -> ConstantIndex {
        self.module.add_constant(constant.into())
    }

    pub fn disassemble(&mut self) {
        for chunk in self.module.chunks() {
            chunk.disassemble(self.module.constants());
            println!();
        }
    }

    // context
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

    fn resolve_local(&self, name: &str) -> Result<Option<StackIndex>, CompileError> {
        self.current_context().resolve_local(name)
    }

    // statement
    pub fn program(&mut self, stmts: &[Stmt]) -> Result<(), CompileError> {
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
        self.statement(block);

        if let Some(stmt) = next {
            let index = self.add_instruction(Instruction::Jump(0));
            self.patch_instruction(next_index);
            self.add_instruction(Instruction::Pop);
            self.statement(stmt.as_ref());
            self.patch_instruction(index);
        } else {
            self.patch_instruction(next_index);
        }
        Ok(())
    }

    // expression
    fn expression(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match *expr {
            Expr::Number(num) => self.number(num),
            Expr::String(ref string) => self.string(string),
            Expr::Boolean(boolean) => self.boolean(boolean),

            Expr::Variable(ref string) => self.variable(string),

            Expr::Binary(ref left, ref op, ref right) => self.binary(left, op, right),
            Expr::Logical(ref left, ref op, ref right) => self.logical(left, op, right),
            Expr::Grouping(ref expr) => self.expression(expr),
            Expr::Unary(ref op, ref expr) => self.unary(op, expr),
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
            &Symbol::And => self.and(left, right),
            &Symbol::Or => self.or(left, right),
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
}