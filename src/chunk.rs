use crate::constant::Constant;

pub type InstructionIndex = usize;
pub type ConstantIndex = usize;
pub type StackIndex = usize;
pub type ChunkIndex = usize;

#[derive(Debug, Clone)]
pub enum Instruction {
    Constant(ConstantIndex),
    True,
    False,
    None,

    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Pow,

    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    GetGlobal(ConstantIndex),
    SetGlobal(ConstantIndex),

    GetLocal(StackIndex),
    SetLocal(StackIndex),

    Pop,

    Return,
    Print,
}

pub struct Chunk {
    pub instructions: Vec<Instruction>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            instructions: vec![],
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}