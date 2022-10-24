use std::cell::RefMut;

use crate::parser::ast::AstNode;

use self::{block::Block, error::CompilerError, instructions::Instruction, program::Program};

pub mod block;
pub mod error;
pub mod instructions;
pub mod program;

#[derive(Debug)]
pub struct Compiler {
    ast: Vec<AstNode>,
    program: Program,
}

impl Compiler {
    pub fn new(ast: Vec<AstNode>) -> Self {
        Self {
            ast,
            program: Program::new(),
        }
    }

    fn get_current_block(&self, id: usize) -> RefMut<Block> {
        self.program.get_mut_block(id)
    }

    fn compile_node(
        &mut self,
        node: &AstNode,
        current_block_id: usize,
    ) -> Result<(), CompilerError> {
        match node {
            AstNode::IntegerLiteral(i, _) => self
                .get_current_block(current_block_id)
                .add_instruction(Instruction::LoadI64(*i)),
            AstNode::FloatLiteral(f, _) => self
                .get_current_block(current_block_id)
                .add_instruction(Instruction::LoadF64(*f)),
            AstNode::BooleanLiteral(b, _) => self
                .get_current_block(current_block_id)
                .add_instruction(Instruction::LoadBool(*b)),
            _ => todo!("compile_node is not implemented for {:?} yet", node),
        }

        Ok(())
    }

    pub fn compile(&mut self) -> Result<Program, CompilerError> {
        let current_block_id = self.program.add_block();

        for node in self.ast.clone() {
            self.compile_node(&node, current_block_id)?;
        }

        Ok(self.program.clone())
    }
}
