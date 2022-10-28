use crate::parser::ast::AstNode;

use self::{
    block::Block,
    error::CompilerError,
    instruction::{Instruction, InstructionKind},
    program::Program,
};

pub mod block;
pub mod instruction;
pub mod program;
pub mod resolver;

mod error;

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

    fn current_block(&mut self) -> &mut Block {
        &mut self.program.blocks[self.current_block]
    }

    fn compile_node(&mut self, node: &AstNode) -> Result<(), CompilerError> {
        match node {
            AstNode::IntegerLiteral(i, _) => self.program.add_instruction_at(
                self.current_block,
                Instruction::new(InstructionKind::LoadI64(*i), node.location()),
            ),
            AstNode::FloatLiteral(f, _) => self.program.add_instruction_at(
                self.current_block,
                Instruction::new(InstructionKind::LoadF64(*f), node.location()),
            ),
            AstNode::BooleanLiteral(b, _) => self.program.add_instruction_at(
                self.current_block,
                Instruction::new(InstructionKind::LoadBool(*b), node.location()),
            ),
            AstNode::StringLiteral(s, _) => {
                let index = self.program.add_string(s);

                self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::LoadConstant(index), node.location()),
                );
            }
            AstNode::FunctionCall { name, .. } => match name.as_str() {
                "__plus" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::Add, node.location()),
                ),
                "__minus" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::Sub, node.location()),
                ),
                "__mult" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::Mul, node.location()),
                ),
                "__div" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::Div, node.location()),
                ),
                "__mod" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::Mod, node.location()),
                ),
                "__not" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::Not, node.location()),
                ),
                "__gt" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::GreaterThan, node.location()),
                ),
                "__gte" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::GreaterThanEquals, node.location()),
                ),
                "__lt" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::LessThan, node.location()),
                ),
                "__lte" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::LessThanEquals, node.location()),
                ),
                "__and" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::And, node.location()),
                ),
                "__or" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::Or, node.location()),
                ),
                "__eqeq" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::Equals, node.location()),
                ),
                "__noteq" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::NotEquals, node.location()),
                ),
                "()__quit" => self.program.add_instruction_at(
                    self.current_block,
                    Instruction::new(InstructionKind::Halt, node.location()),
                ),
                _ => {
                    let index = match self.program.functions.get(name) {
                        Some(i) => *i,
                        None => return Err(CompilerError::UnknownFunction(name.clone())),
                    };

                    self.program.add_instruction_at(
                        self.current_block,
                        Instruction::new(InstructionKind::Call(index), node.location()),
                    );
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
                    Instruction::new(InstructionKind::Halt, node.location())
                } else {
                    Instruction::new(InstructionKind::Return, node.location())
                };

                self.program
                    .add_instruction_at(self.current_block, instruction);

                self.current_block = old_block;
            }
            AstNode::IfExpression {
                then_branch,
                else_branch,
                ..
            } => {
                // let then_block_entry = self.current_block().instructions.len();
                let instruction_to_patch = self
                    .current_block()
                    .add_instruction(Instruction::new(InstructionKind::Patch, node.location()));

                for node in &then_branch.nodes {
                    self.compile_node(node)?;
                }

                let end_then_branch_instruction = self
                    .current_block()
                    .add_instruction(Instruction::new(InstructionKind::Patch, node.location()));

                let else_block_entry = self.current_block().instructions.len();

                match else_branch {
                    None => {}
                    Some(else_branch) => {
                        for node in &else_branch.nodes {
                            self.compile_node(node)?;
                        }
                    }
                }

                self.current_block().patch_instruction(
                    instruction_to_patch,
                    Instruction::new(
                        InstructionKind::JumpIfFalse(else_block_entry),
                        node.location(),
                    ),
                );

                let index = self.current_block().instructions.len();
                self.current_block().patch_instruction(
                    end_then_branch_instruction,
                    Instruction::new(InstructionKind::Jump(index), node.location()),
                );
            }
            AstNode::WhileExpression {
                condition, body, ..
            } => {
                let condition_block_entry = self.current_block().instructions.len();

                for node in &condition.nodes {
                    self.compile_node(node)?;
                }

                let instruction_to_patch = self
                    .current_block()
                    .add_instruction(Instruction::new(InstructionKind::Patch, node.location()));

                for node in &body.nodes {
                    self.compile_node(node)?;
                }

                self.current_block().add_instruction(Instruction::new(
                    InstructionKind::Jump(condition_block_entry),
                    node.location(),
                ));

                let index = self.current_block().instructions.len();
                self.current_block().patch_instruction(
                    instruction_to_patch,
                    Instruction::new(InstructionKind::JumpIfFalse(index), node.location()),
                );
            }
            AstNode::Identifier(name, _) => {
                match name.as_str() {
                    "dup" => self
                        .current_block()
                        .add_instruction(Instruction::new(InstructionKind::Dup, node.location())),
                    "drop" => self
                        .current_block()
                        .add_instruction(Instruction::new(InstructionKind::Drop, node.location())),
                    "print" => self
                        .current_block()
                        .add_instruction(Instruction::new(InstructionKind::Print, node.location())),
                    "swap" => self
                        .current_block()
                        .add_instruction(Instruction::new(InstructionKind::Swap, node.location())),
                    "over" => self
                        .current_block()
                        .add_instruction(Instruction::new(InstructionKind::Over, node.location())),
                    "???" => self.current_block().add_instruction(Instruction::new(
                        InstructionKind::DebugStack,
                        node.location(),
                    )),
                    _ => {
                        if self.program.functions.contains_key(name) {
                            let index = self.program.functions[name];

                            self.current_block().add_instruction(Instruction::new(
                                InstructionKind::Call(index),
                                node.location(),
                            ));

                            return Ok(());
                        }

                        todo!("compile_node::AstNode::Identifier not yet implemented for non function identifiers `{}`", name);
                    }
                };
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
