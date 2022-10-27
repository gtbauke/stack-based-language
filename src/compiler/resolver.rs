use crate::parser::ast::AstNode;

use super::{error::ResolverError, program::Program};

#[derive(Debug)]
pub struct Resolver {
    program: Program,
    current_block: usize,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            program: Program::new(),
            current_block: 0,
        }
    }

    fn resolve_node(&mut self, node: &AstNode) -> Result<(), ResolverError> {
        match node {
            AstNode::FunctionDeclaration { name, body, .. } => {
                self.current_block = self.program.add_block();
                self.program.add_function(name, self.current_block);

                for node in &body.nodes {
                    self.resolve_node(node)?;
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn resolve(&mut self, ast: Vec<AstNode>) -> Result<Program, ResolverError> {
        for node in &ast {
            self.resolve_node(node)?;
        }

        Ok(self.program.clone())
    }
}
