use crate::parser::ast::AstNode;

use self::{error::CompilerError, instruction::Instruction, program::Program};

pub mod instruction;
pub mod program;

mod error;

// TODO: hoist functions

#[derive(Debug, Clone)]
pub struct Compiler {
    program: Program,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            program: Program::new(),
        }
    }

    fn compile_node(&mut self, node: &AstNode) -> Result<(), CompilerError> {
        match node {
            AstNode::IntegerLiteral(i, _) => self.program.add_instruction(Instruction::LoadI64(*i)),
            AstNode::FloatLiteral(f, _) => self.program.add_instruction(Instruction::LoadF64(*f)),
            AstNode::BooleanLiteral(b, _) => {
                self.program.add_instruction(Instruction::LoadBool(*b))
            }
            AstNode::StringLiteral(s, _) => {
                let index = self.program.add_string(s);

                self.program
                    .add_instruction(Instruction::LoadConstant(index));
            }
            AstNode::FunctionCall { name, .. } => match name.as_str() {
                "__plus" => self.program.add_instruction(Instruction::Add),
                "__minus" => self.program.add_instruction(Instruction::Sub),
                "__mult" => self.program.add_instruction(Instruction::Mul),
                "__div" => self.program.add_instruction(Instruction::Div),
                "__not" => self.program.add_instruction(Instruction::Not),
                "__gt" => self.program.add_instruction(Instruction::GreaterThan),
                "__gte" => self.program.add_instruction(Instruction::GreaterThanEquals),
                "__lt" => self.program.add_instruction(Instruction::LessThan),
                "__lte" => self.program.add_instruction(Instruction::LessThanEquals),
                "__and" => self.program.add_instruction(Instruction::And),
                "__or" => self.program.add_instruction(Instruction::Or),
                "__eqeq" => self.program.add_instruction(Instruction::Equals),
                "__noteq" => self.program.add_instruction(Instruction::NotEquals),
                _ => {
                    let index = match self.program.functions.get(name) {
                        Some(i) => *i,
                        None => return Err(CompilerError::UnknownFunction(name.clone())),
                    };

                    self.program.add_instruction(Instruction::Call(index));
                }
            },
            AstNode::FunctionDeclaration { name, body, .. } => {
                let entry_point = self.program.instructions.len();
                self.program.add_function(name, entry_point);

                for node in &body.nodes {
                    self.compile_node(node)?;
                }

                let instruction = if name == "main" {
                    Instruction::Halt
                } else {
                    Instruction::Return
                };

                self.program.add_instruction(instruction);
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
