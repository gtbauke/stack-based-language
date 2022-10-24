use super::instructions::Instruction;

#[derive(Debug, Clone)]
pub struct Block {
    pub instructions: Vec<Instruction>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.push(self.instructions.len() as u8);

        for instruction in self.instructions.iter() {
            bytes.extend_from_slice(&instruction.to_bytes());
        }

        bytes
    }
}
