use super::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub entry_point: usize,

    pub strings: Vec<String>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            instructions: Vec::new(),
            entry_point: 0,

            strings: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn set_entry_point(&mut self, entry_point: usize) {
        self.entry_point = entry_point;
    }

    pub fn add_string(&mut self, string: &str) -> usize {
        match self.strings.iter().position(|s| s == string) {
            Some(index) => index,
            None => {
                self.strings.push(string.to_string());
                self.strings.len() - 1
            }
        }
    }
}
