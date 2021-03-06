/// Debug code that will print out the instructions of each chunk with their
/// given offsets and operands.
use crate::bytecode::chunk::Chunk;
use crate::bytecode::chunk::Instruction;
use crate::bytecode::constant::Constant;
use crate::bytecode::module::ConstantPool;

impl Chunk {
    pub fn disassemble(&self, pool: &ConstantPool) {
        println!(
            "{:<8}\t\t{:<16}\t{:<16}",
            "OFFSET", "INSTRUCTION", "OPERAND"
        );

        for (index, instruction) in self.instructions.clone().into_iter().enumerate() {
            match instruction {
                Instruction::Constant(offset) => {
                    constant_instruction(index, Instruction::Constant(offset), pool.get(offset))
                }
                i => simple_instruction(index, i),
            }
        }
    }
}

fn simple_instruction(offset: usize, instruction: Instruction) {
    println!(
        "{:<#008x}\t\t{:<16}\t{:<16}",
        offset,
        format!("{:?}", instruction),
        ""
    );
}

fn constant_instruction(offset: usize, instruction: Instruction, constant: &Constant) {
    println!(
        "{:#008x}\t\t{:<16}\t{:<16}",
        offset,
        format!("{:?}", instruction),
        format!("{:?}", constant)
    );
}
