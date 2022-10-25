use std::{cell::RefCell, rc::Rc};

use super::instructions::Instruction;

#[derive(Debug, Clone)]
pub struct Block {
    pub instructions: Vec<Rc<RefCell<Instruction>>>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(Rc::new(RefCell::new(instruction)));
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.push(self.instructions.len() as u8);

        for instruction in self.instructions.iter() {
            bytes.extend_from_slice(&instruction.borrow().to_bytes());
        }

        bytes
    }
}
