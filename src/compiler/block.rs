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

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}
