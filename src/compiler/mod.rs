use crate::parser::ast::AstNode;

use self::{error::CompilerError, instruction::Instruction, program::Program};

pub mod block;
pub mod instruction;
pub mod program;
pub mod resolver;

mod error;

// TODO: hoist functions

#[derive(Debug, Clone)]
pub struct Compiler {
    program: Program,
    current_block: usize,
}

impl Compiler {
    pub fn new(resolved_program: Program) -> Self {
        Compiler {
            program: resolved_program,
            current_block: 0,
        }
    }

    fn compile_node(&mut self, node: &AstNode) -> Result<(), CompilerError> {
        match node {
            AstNode::IntegerLiteral(i, _) => self
                .program
                .add_instruction_at(self.current_block, Instruction::LoadI64(*i)),
            AstNode::FloatLiteral(f, _) => self
                .program
                .add_instruction_at(self.current_block, Instruction::LoadF64(*f)),
            AstNode::BooleanLiteral(b, _) => self
                .program
                .add_instruction_at(self.current_block, Instruction::LoadBool(*b)),
            AstNode::StringLiteral(s, _) => {
                let index = self.program.add_string(s);

                self.program
                    .add_instruction_at(self.current_block, Instruction::LoadConstant(index));
            }
            AstNode::FunctionCall { name, .. } => match name.as_str() {
                "__plus" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::Add),
                "__minus" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::Sub),
                "__mult" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::Mul),
                "__div" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::Div),
                "__not" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::Not),
                "__gt" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::GreaterThan),
                "__gte" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::GreaterThanEquals),
                "__lt" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::LessThan),
                "__lte" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::LessThanEquals),
                "__and" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::And),
                "__or" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::Or),
                "__eqeq" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::Equals),
                "__noteq" => self
                    .program
                    .add_instruction_at(self.current_block, Instruction::NotEquals),
                _ => {
                    let index = match self.program.functions.get(name) {
                        Some(i) => *i,
                        None => return Err(CompilerError::UnknownFunction(name.clone())),
                    };

                    self.program
                        .add_instruction_at(self.current_block, Instruction::Call(index));
                }
            },
            AstNode::FunctionDeclaration { name, body, .. } => {
                let entry_point = match self.program.functions.get(name) {
                    Some(index) => *index,
                    None => unreachable!(),
                };

                let old_block = self.current_block;
                self.current_block = entry_point;

                for node in &body.nodes {
                    self.compile_node(node)?;
                }

                let instruction = if name == "main" {
                    Instruction::Halt
                } else {
                    Instruction::Return
                };

                self.program
                    .add_instruction_at(self.current_block, instruction);

                self.current_block = old_block;
            }
            _ => todo!("compile_node is not implemented for {:?} yet", node),
        }

        Ok(())
    }

    pub fn compile(&mut self, ast: Vec<AstNode>) -> Result<Program, CompilerError> {
        for node in &ast {
            self.compile_node(node)?;
        }

        Ok(self.program.clone())
    }
}
