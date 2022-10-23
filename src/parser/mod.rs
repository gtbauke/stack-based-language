use crate::{
    lexer::Lexer,
    token::{Token, TokenKind},
};

use self::ast::AstNode;

pub mod ast;

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer,

    current: Option<Token>,
    previous: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Parser<'a> {
        Parser {
            lexer,
            current: None,
            previous: None,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current
            .as_ref()
            .map_or(false, |token| token.kind == TokenKind::EOF)
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();
        self.current = Some(self.lexer.next());
    }

    fn get_function_name(&self, kind: &TokenKind) -> Option<String> {
        match kind {
            TokenKind::Plus => Some("__plus".to_string()),
            TokenKind::Minus => Some("__minus".to_string()),
            TokenKind::Star => Some("__mult".to_string()),
            TokenKind::Slash => Some("__div".to_string()),
            TokenKind::Equal => Some("__eq".to_string()),
            TokenKind::Bang | TokenKind::Not => Some("__not".to_string()),
            TokenKind::Greater => Some("__gt".to_string()),
            TokenKind::GreaterEqual => Some("__gte".to_string()),
            TokenKind::Less => Some("__lt".to_string()),
            TokenKind::LessEqual => Some("__lte".to_string()),
            TokenKind::And => Some("__and".to_string()),
            TokenKind::Or => Some("__or".to_string()),
            TokenKind::EqualEqual => Some("__eqeq".to_string()),
            TokenKind::BangEqual => Some("__noteq".to_string()),
            _ => None,
        }
    }

    fn parse_expression(&mut self) -> AstNode {
        match self.current.as_ref() {
            Some(token) => {
                let location = &token.location;

                match &token.kind {
                    TokenKind::Integer(i) => AstNode::IntegerLiteral(*i, location.clone()),
                    TokenKind::Float(f) => AstNode::FloatLiteral(*f, location.clone()),
                    TokenKind::String(s) => AstNode::StringLiteral(s.to_string(), location.clone()),
                    TokenKind::True => AstNode::BooleanLiteral(true, location.clone()),
                    TokenKind::False => AstNode::BooleanLiteral(false, location.clone()),
                    TokenKind::Identifier(s) => {
                        AstNode::Identifier(s.to_string(), location.clone())
                    }
                    TokenKind::Plus
                    | TokenKind::Minus
                    | TokenKind::Star
                    | TokenKind::Slash
                    | TokenKind::Equal
                    | TokenKind::Bang
                    | TokenKind::Not
                    | TokenKind::Greater
                    | TokenKind::GreaterEqual
                    | TokenKind::Less
                    | TokenKind::LessEqual
                    | TokenKind::And
                    | TokenKind::Or
                    | TokenKind::EqualEqual
                    | TokenKind::BangEqual => {
                        let name = self.get_function_name(&token.kind);

                        match name {
                            None => panic!("Invalid operator"),
                            Some(name) => AstNode::FunctionCall {
                                name,
                                location: location.clone(),
                            },
                        }
                    }
                    _ => todo!("parse_expression not implemented for {:?} yet", token),
                }
            }
            None => {
                // TODO: error handling that does not panic
                panic!("Expected expression, got EOF");
            }
        }
    }

    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut nodes = Vec::new();

        while !self.is_at_end() {
            self.advance();

            if self
                .current
                .as_ref()
                .map_or(false, |token| token.kind == TokenKind::EOF)
            {
                break;
            }

            nodes.push(self.parse_expression());
        }

        nodes
    }
}
