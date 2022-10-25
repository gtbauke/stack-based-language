use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use super::block::Block;

/*
Program layout:

Header:
2 bytes -> Magic Number
1 byte -> Version
1 byte -> Block Count // TODO: maybe we should use 2 bytes for this
1 byte -> Entry Point (Header Size + Block Entry Point)

Strings:
1 byte -> String Length
n bytes -> String

Blocks:
n bytes -> Blocks

Block:
1 byte -> Instruction Count // TODO: maybe we should use 2 bytes for this
n bytes -> Instructions

*/

const MAGIC_NUMBER: u16 = 0x4C4F;

#[derive(Debug, Clone)]
pub struct Program {
    pub blocks: Vec<Rc<RefCell<Block>>>,
    pub entry_point: usize,

    pub strings: Vec<String>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            entry_point: 0,

            strings: Vec::new(),
        }
    }

    pub fn add_block(&mut self) -> usize {
        self.blocks.push(Rc::new(RefCell::new(Block::new())));
        self.blocks.len() - 1
    }

    pub fn internalize_string(&mut self, string: &str) -> usize {
        match self.strings.iter().position(|s| s == string) {
            Some(id) => id,
            None => {
                self.strings.push(string.to_string());
                self.strings.len() - 1
            }
        }
    }

    pub fn get_mut_block(&self, block_id: usize) -> RefMut<Block> {
        self.blocks[block_id].as_ref().borrow_mut()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend_from_slice(&MAGIC_NUMBER.to_le_bytes());
        bytes.push(0x01); // Version

        let string_block_size = self.strings.iter().fold(0, |acc, s| acc + s.len() + 1);

        bytes.push(self.blocks.len() as u8);
        bytes.push(string_block_size as u8 + self.entry_point as u8);

        for string in &self.strings {
            bytes.push(string.len() as u8);
            bytes.extend_from_slice(string.as_bytes());
        }

        for block in self.blocks.iter() {
            bytes.extend_from_slice(&block.as_ref().borrow().to_bytes());
        }

        bytes
    }
}
