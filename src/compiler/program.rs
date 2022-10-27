use std::collections::HashMap;

use super::{block::Block, instruction::Instruction};

#[derive(Debug, Clone)]
pub struct Program {
    pub blocks: Vec<Block>,
    pub entry_point: usize,

    pub strings: Vec<String>,
    pub functions: HashMap<String, usize>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            blocks: Vec::new(),
            entry_point: 0,

            strings: Vec::new(),
            functions: HashMap::new(),
        }
    }

    pub fn add_block(&mut self) -> usize {
        self.blocks.push(Block::new());
        self.blocks.len() - 1
    }

    pub fn add_instruction_at(&mut self, block_id: usize, instruction: Instruction) {
        self.blocks[block_id].add_instruction(instruction);
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

    pub fn add_function(&mut self, name: &str, block_id: usize) {
        self.functions.insert(name.to_string(), block_id);
    }
}
