use crate::chunk::{Chunk, Instruction, ConstantIndex, ChunkIndex, StackIndex};
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

    pub fn resolve_local(&mut self, name: &str) -> Result<Option<StackIndex>, CompileError> {
        if let Some(local) = self.locals.get(name) {
            Ok(Some(local.location()))
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
    pub fn add_instruction(&mut self, instruction: Instruction) {
        let context = self.contexts.last().expect("expected some compiler context");
        let index = context.chunk_index;
        self.module.add_instruction(index, instruction);
    }

    pub fn add_constant<C: Into<Constant>>(&mut self, constant: C) -> ConstantIndex {
        self.module.add_constant(constant.into())
    }

    pub fn disassemble(&mut self) {
        for chunk in self.module.chunks() {
            chunk.disassemble(self.module.constants());
            println!();
        }
    }

    // context
    pub fn current_context(&mut self) -> &mut CompilerContext {
        self.contexts.last_mut().expect("expected a compiler context to exist")
    }

    // statement
    pub fn program(&mut self, stmts: &[Stmt]) -> Result<(), CompileError> {
        for stmt in stmts {
            self.statement(stmt);
        }
        self.add_instruction(Instruction::Return);
        Ok(())
    }

    fn statement(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        match stmt {
            Stmt::Assign(ref name, ref expr) => self.assign(name, expr),
            Stmt::Block(ref stmts) => self.block(stmts),
            Stmt::Print(ref expr) => self.print(expr),
            Stmt::Expression(ref expr) => self.expression(expr),
            _ => Err(CompileError::from(format!("Unrecognized statement {:?}", stmt)))
        };
        Ok(())
    }

    fn block(&mut self, stmts: &[Stmt]) -> Result<(), CompileError> {
        self.current_context().locals.begin_scope();
        for stmt in stmts {
            self.statement(stmt);
        }
        self.current_context().locals.end_scope();
        Ok(())
    }

    fn print(&mut self, expr: &Expr) -> Result<(), CompileError> {
        self.expression(expr);
        self.add_instruction(Instruction::Print);
        Ok(())
    }

    // expression
    fn expression(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match *expr {
            Expr::Number(num) => self.number(num),
            Expr::String(ref string) => self.string(string),
            Expr::Boolean(boolean) => self.boolean(boolean),

            Expr::Variable(ref string) => self.variable(string),
            // Expr::Assign(ref string, ref expr) => self.assign(string, expr),

            Expr::Binary(ref left, ref op, ref right) => self.binary(left, op, right),
            Expr::Grouping(ref expr) => self.expression(expr),
            Expr::Unary(ref op, ref expr) => self.unary(op, expr),
            _ => Err(CompileError::from("Unexpected expression!"))
        }
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

    fn variable(&mut self, name: &str) -> Result<(), CompileError> {
        if let Some(local) = self.current_context().resolve_local(name)? {
            self.add_instruction(Instruction::GetLocal(local));
        } else {
            let constant = self.add_constant(name);
            self.add_instruction(Instruction::GetGlobal(constant));
        }
        Ok(())
    }

    fn assign(&mut self, name: &str, expr: &Expr) -> Result<(), CompileError> {
        self.expression(expr)?;

        if self.current_context().scope_depth() > 0 {
            let local = match self.current_context().resolve_local(name)? {
                Some(local) => local,
                None => self.current_context().locals.insert(name).location()
            };
            self.add_instruction(Instruction::SetLocal(local));
        } else {
            let constant = self.add_constant(name);
            self.add_instruction(Instruction::SetGlobal(constant));
        }
        Ok(())
    }

    fn binary(&mut self, left: &Expr, op: &Symbol, right: &Expr) -> Result<(), CompileError> {
        self.expression(left);
        self.expression(right);

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
        }

        Ok(())
    }

    fn unary(&mut self, op: &Symbol, expr: &Expr) -> Result<(), CompileError> {
        self.expression(expr);
        match *op {
            Symbol::Not => self.add_instruction(Instruction::Not),
            _ => panic!("Unknown unary Operator")
        }
        Ok(())
    }
}