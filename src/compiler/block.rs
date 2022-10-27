use super::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Block {
    pub instructions: Vec<Instruction>,
}

impl Block {
    pub fn new() -> Self {
        Block {
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> usize {
        self.instructions.push(instruction);
        self.instructions.len() - 1
    }

    pub fn patch_instruction(&mut self, index: usize, instruction: Instruction) {
        self.instructions[index] = instruction;
    }
}
