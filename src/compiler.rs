use crate::ast::Expr;
use crate::ast::Stmt;
use crate::chunk::Chunk;
use crate::chunk::ConstantIndex;
use crate::chunk::Instruction;
use crate::constant::Constant;
use crate::error::CompileError;
use crate::local::Locals;
use crate::local::Upvalue;
use crate::memory::Distance;
use crate::tokens::Symbol;
use crate::tokens::Token;
use string_interner::StringInterner;
use string_interner::Sym;

use crate::module::Module;

#[derive(Debug, PartialEq)]
pub enum ContextType {
    Function,
    Script,
}

#[derive(Debug)]
pub struct CompilerContext {
    pub context_type: ContextType,
    pub enclosing: usize,
    pub chunk_index: usize,
    pub locals: Locals,
    pub upvalues: Vec<Upvalue>,
}

impl CompilerContext {
    pub fn new(context_type: ContextType, enclosing: usize, chunk_index: usize) -> Self {
        CompilerContext {
            context_type,
            enclosing,
            chunk_index,
            locals: Locals::new(),
            upvalues: vec![],
        }
    }

    pub fn resolve_local(&mut self, sym: &Sym) -> Result<Option<usize>, CompileError> {
        if let Some(local) = self.locals.get(*sym) {
            if local.is_initialized {
                Ok(Some(local.slot))
            } else {
                Err(CompileError::UndefinedVariable)
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub struct Compiler<'a> {
    module: Module<'a>,
    contexts: Vec<CompilerContext>,
}

impl<'a> Compiler<'a> {
    pub fn new(strings: &'a StringInterner<Sym>) -> Self {
        let mut module = Module::new(strings);
        let chunk_index = module.add_chunk();

        let mut contexts: Vec<CompilerContext> = vec![];
        contexts.push(CompilerContext::new(ContextType::Script, 0, chunk_index));

        Compiler { module, contexts }
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

    fn resolve_upvalue(&mut self, sym: &Sym) -> Result<Option<usize>, CompileError> {
        let enclosing = self.current_context().enclosing;

        if enclosing == 0 {
            return Ok(None);
        }

        if let Some(local) = self.contexts[enclosing].resolve_local(sym)? {
            self.current_context_mut()
                .upvalues
                .push(Upvalue::new(local, true, false));
        }

        Ok(None)
    }

    fn resolve_local(&mut self, sym: &Sym) -> Result<Option<usize>, CompileError> {
        self.current_context_mut().resolve_local(sym)
    }

    pub fn compile(&mut self, statements: &[Stmt]) -> Result<&Module, CompileError> {
        self.compile_program(statements)?;

        println!("Constants: {:?}", self.module.constants);
        for chunk in &self.module.chunks {
            chunk.disassemble(&self.module.constants);
            println!();
        }

        Ok(&self.module)
    }

    fn compile_program(&mut self, statements: &[Stmt]) -> Result<(), CompileError> {
        for statement in statements {
            self.compile_statement(statement)?;
        }
        self.add_instruction(Instruction::Return);
        Ok(())
    }

    fn compile_statement(&mut self, statement: &Stmt) -> Result<(), CompileError> {
        match statement {
            Stmt::Expression(ref expr) => self.compile_expression(expr),
            Stmt::Print(ref expr) => self.compile_print(expr),
            Stmt::Return(ref expr) => self.compile_return(expr),
            Stmt::Assign(ref sym, ref expr) => self.compile_assign(sym, expr),
            Stmt::Block(ref statements) => self.compile_block(statements),
            Stmt::If(ref condition, ref body, ref next) => self.compile_if(condition, body, next),
            Stmt::While(ref condition, ref body) => self.compile_while(condition, body),
            Stmt::Function(ref sym, ref params, ref body) => {
                self.compile_function(sym, params, body)
            }
        }?;

        Ok(())
    }

    fn compile_expression(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match expr {
            Expr::Number(ref distance) => self.compile_number(distance),
            Expr::String(ref sym) => self.compile_string(sym),
            Expr::Boolean(ref boolean) => self.compile_boolean(boolean),
            Expr::None => self.compile_none(),
            Expr::Variable(ref sym) => self.compile_variable(sym),
            Expr::List(ref elements) => self.compile_list(elements),
            Expr::Index(ref sym, ref expr) => self.compile_index(sym, expr),
            Expr::Slice(ref start, ref stop, ref step) => self.compile_slice(start, stop, step),
            Expr::Binary(ref left, ref op, ref right) => self.compile_binary(left, op, right),
            Expr::Logical(ref left, ref op, ref right) => self.compile_logical(left, op, right),
            Expr::Grouping(ref expr) => self.compile_grouping(expr),
            Expr::Unary(ref op, ref right) => self.compile_unary(op, right),
            Expr::Call(ref callee, ref arguments) => self.compile_call(callee, arguments),
        }?;

        Ok(())
    }

    fn compile_number(&mut self, distance: &Distance) -> Result<(), CompileError> {
        let constant = self.add_constant(Constant::from(distance));
        self.add_instruction(Instruction::Constant(constant));
        Ok(())
    }

    fn compile_string(&mut self, sym: &Sym) -> Result<(), CompileError> {
        let constant = self.add_constant(Constant::from(sym));
        self.add_instruction(Instruction::Constant(constant));
        Ok(())
    }

    fn compile_boolean(&mut self, boolean: &bool) -> Result<(), CompileError> {
        if *boolean {
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

    fn compile_variable(&mut self, sym: &Sym) -> Result<(), CompileError> {
        // if local :-
        if let Some(local) = self.resolve_local(sym)? {
            self.add_instruction(Instruction::GetLocal(local));
        }
        // else if upvalue :-
        else if let Some(upvalue) = self.resolve_upvalue(sym)? {
            self.add_instruction(Instruction::GetUpvalue(upvalue));
        }
        // else global
        else {
            let constant = self.add_constant(Constant::from(sym));
            self.add_instruction(Instruction::GetGlobal(constant));
        }
        Ok(())
    }

    fn compile_list(&mut self, expressions: &[Expr]) -> Result<(), CompileError> {
        Ok(())
    }

    fn compile_index(&mut self, sym: &Sym, expression: &Expr) -> Result<(), CompileError> {
        Ok(())
    }

    fn compile_slice(
        &mut self,
        start: &Option<Box<Expr>>,
        stop: &Option<Box<Expr>>,
        step: &Option<Box<Expr>>,
    ) -> Result<(), CompileError> {
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

        match op.symbol {
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
            _ => return Err(CompileError::UnexpectedBinaryOperator(op.clone())),
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
            _ => Err(CompileError::UnexpectedLogicalOperator(op.clone())),
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

    fn compile_unary(&mut self, op: &Token, right: &Expr) -> Result<(), CompileError> {
        self.compile_expression(right)?;
        match op.symbol {
            Symbol::Not => self.add_instruction(Instruction::Not),
            _ => return Err(CompileError::UnexpectedUnaryOperator(op.clone())),
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

    fn compile_print(&mut self, expr: &Expr) -> Result<(), CompileError> {
        self.compile_expression(expr)?;
        self.add_instruction(Instruction::Print);
        Ok(())
    }

    fn compile_return(&mut self, expr: &Option<Box<Expr>>) -> Result<(), CompileError> {
        if self.current_context().context_type == ContextType::Script {
            Err(CompileError::ReturnInScript)
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

    fn compile_assign(&mut self, sym: &Sym, expr: &Expr) -> Result<(), CompileError> {
        if self.current_context().locals.depth > 0 {
            self.current_context_mut().locals.insert(sym);
        }

        self.compile_expression(expr)?;

        if self.current_context().locals.depth > 0 {
            self.current_context_mut().locals.mark_initialized();
        } else {
            let constant = self.add_constant(Constant::from(sym));
            self.add_instruction(Instruction::SetGlobal(constant));
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
        sym: &Sym,
        params: &[Sym],
        body: &Stmt,
    ) -> Result<(), CompileError> {
        Ok(())
    }
}
