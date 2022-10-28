use crate::token::location::Location;

#[derive(Debug, Clone)]
pub struct Block {
    pub nodes: Vec<AstNode>,
}

impl Block {
    pub fn new(nodes: Vec<AstNode>) -> Block {
        Block { nodes }
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    IntegerLiteral(i64, Location),
    FloatLiteral(f64, Location),
    StringLiteral(String, Location),
    BooleanLiteral(bool, Location),
    Identifier(String, Location),

    FunctionCall {
        name: String,
        location: Location,
    },

    IfExpression {
        then_branch: Block,
        else_branch: Option<Block>,
        location: Location,
    },

    WhileExpression {
        condition: Block,
        body: Block,
        location: Location,
    },

    FunctionDeclaration {
        name: String,
        body: Block,
        location: Location,
    },

    LetDeclaration {
        bindings: Vec<String>,
        location: Location,
    },
}

impl AstNode {
    pub fn location(&self) -> &Location {
        match self {
            AstNode::IntegerLiteral(_, location) => location,
            AstNode::FloatLiteral(_, location) => location,
            AstNode::StringLiteral(_, location) => location,
            AstNode::BooleanLiteral(_, location) => location,
            AstNode::Identifier(_, location) => location,
            AstNode::FunctionCall { location, .. } => location,
            AstNode::IfExpression { location, .. } => location,
            AstNode::WhileExpression { location, .. } => location,
            AstNode::FunctionDeclaration { location, .. } => location,
            AstNode::LetDeclaration { location, .. } => location,
        }
    }
}
