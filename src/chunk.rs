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
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    GetGlobal(ConstantIndex),
    SetGlobal(ConstantIndex),

    GetLocal(StackIndex),
    SetLocal(StackIndex),

    JumpIfTrue(StackIndex),
    JumpIfFalse(StackIndex),
    Jump(StackIndex),

    Call(usize),

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

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> InstructionIndex {
        self.instructions.push(instruction);
        self.instructions.len() - 1
    }

    pub fn patch_instruction(&mut self, index: InstructionIndex) {
        let current = self.instructions.len();
        self.patch_instruction_to(index, current);
    }

    pub fn patch_instruction_to(&mut self, index: InstructionIndex, to: InstructionIndex) {
        match self.instructions[index] {
            Instruction::JumpIfTrue(ref mut placeholder) => *placeholder = to,
            Instruction::JumpIfFalse(ref mut placeholder) => *placeholder = to,
            Instruction::Jump(ref mut placeholder) => *placeholder = to,
            _ => panic!(format!("Cannot patch instruction {:?}, ", self.instructions[index])),
        };
    }
}