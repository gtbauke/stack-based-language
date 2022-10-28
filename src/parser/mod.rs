use core::panic;

use crate::token::{location::Location, Token, TokenKind};

use self::ast::{AstNode, Block};

pub mod ast;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn get_function_name(&self, kind: &TokenKind) -> Option<String> {
        match kind {
            TokenKind::Plus => Some("__plus".to_string()),
            TokenKind::Minus => Some("__minus".to_string()),
            TokenKind::Star => Some("__mult".to_string()),
            TokenKind::Slash => Some("__div".to_string()),
            TokenKind::Percent => Some("__mod".to_string()),
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

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.current + offset)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn previous_location(&self) -> Location {
        match self.previous() {
            Some(token) => token.location.clone(),
            None => panic!("No previous location"),
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        self.current += 1;
        self.tokens.get(self.current - 1)
    }

    fn is_at_end(&self) -> bool {
        self.peek(0)
            .map_or(false, |token| token.kind == TokenKind::EOF)
    }

    fn consume_identifier(&mut self) -> Option<String> {
        match self.peek(0) {
            Some(token) => match token.kind.clone() {
                TokenKind::Identifier(lexeme) => {
                    self.advance();
                    Some(lexeme.clone())
                }
                _ => None,
            },
            None => None,
        }
    }

    fn consume(&mut self, kind: TokenKind) -> Option<&Token> {
        if self.peek(0).map_or(false, |token| token.kind == kind) {
            self.advance()
        } else {
            panic!("Expected token {:?}, got {:?}", kind, self.peek(0));
        }
    }

    fn consume_any_of(&mut self, kinds: &[TokenKind]) -> Option<&Token> {
        if self
            .peek(0)
            .map_or(false, |token| kinds.contains(&token.kind))
        {
            self.advance()
        } else {
            panic!(
                "Expected token {:?}, got {:?}",
                kinds,
                self.peek(0).map(|token| token.kind.clone())
            );
        }
    }

    fn parse_block(&mut self) -> Block {
        let mut nodes = Vec::new();

        loop {
            match self.peek(0) {
                Some(token) => match &token.kind {
                    TokenKind::End => break,
                    _ => nodes.push(self.parse_node()),
                },
                None => break,
            }
        }

        self.consume(TokenKind::End);
        Block::new(nodes)
    }

    fn parse_function_definition(&mut self) -> AstNode {
        let location = self.previous_location();
        let name = match self.consume_identifier() {
            Some(lexeme) => lexeme,
            None => panic!("Unable to get function name"),
        };

        self.consume(TokenKind::Do);

        let body = self.parse_block();
        let location =
            location.combine(body.nodes.last().map_or(&location, |node| &node.location()));

        AstNode::FunctionDeclaration {
            name,
            body,
            location,
        }
    }

    fn parse_while_condition(&mut self) -> Block {
        let mut nodes = Vec::new();

        loop {
            match self.peek(0) {
                Some(token) => match &token.kind {
                    TokenKind::Do => break,
                    _ => nodes.push(self.parse_node()),
                },
                None => break,
            }
        }

        self.consume(TokenKind::Do);
        Block::new(nodes)
    }

    fn parse_while_expression(&mut self) -> AstNode {
        let location = self.previous_location();
        let condition = self.parse_while_condition();
        let body = self.parse_block();

        let location =
            location.combine(body.nodes.last().map_or(&location, |node| &node.location()));

        AstNode::WhileExpression {
            condition,
            body,
            location,
        }
    }

    fn parse_operator(&mut self) -> AstNode {
        let token = match self.previous() {
            Some(token) => token,
            None => panic!("No previous token"),
        };

        let function_name = self.get_function_name(&token.kind);
        let function_name = match function_name {
            Some(name) => name,
            None => panic!("Unable to get function name"),
        };

        AstNode::FunctionCall {
            name: function_name,
            location: token.location.clone(),
        }
    }

    fn parse_if_block(&mut self) -> Block {
        let mut nodes = Vec::new();

        loop {
            match self.peek(0) {
                Some(token) => match &token.kind {
                    TokenKind::Else | TokenKind::End => break,
                    _ => nodes.push(self.parse_node()),
                },
                None => break,
            }
        }

        Block::new(nodes)
    }

    fn parse_if_expression(&mut self) -> AstNode {
        let location = self.previous_location();
        let then_branch = self.parse_if_block();

        let terminator = self.consume_any_of(&[TokenKind::Else, TokenKind::End]);
        let else_branch = match terminator {
            None => panic!(
                "Expected token {:?}, got {:?}",
                [TokenKind::Else, TokenKind::End],
                self.peek(0)
            ),
            Some(token) => match token.kind {
                TokenKind::Else => Some(self.parse_block()),
                TokenKind::End => {
                    self.consume(TokenKind::End);
                    None
                }
                _ => panic!(
                    "Expected token {:?}, got {:?}",
                    [TokenKind::Else, TokenKind::End],
                    self.peek(0)
                ),
            },
        };

        let then_branch_location = then_branch
            .nodes
            .last()
            .map_or(&location, |node| &node.location());

        let location =
            location.combine(else_branch.as_ref().map_or(&then_branch_location, |block| {
                block
                    .nodes
                    .last()
                    .map_or(&location, |node| &node.location())
            }));

        AstNode::IfExpression {
            then_branch,
            else_branch,
            location,
        }
    }

    fn parse_function_call(&mut self) -> AstNode {
        let location = self.previous_location();
        let name = match self.consume_identifier() {
            Some(lexeme) => lexeme,
            None => panic!("Unable to get function name"),
        };

        AstNode::FunctionCall { name, location }
    }

    fn parse_node(&mut self) -> AstNode {
        let token = self.advance();

        match token {
            None => panic!("Unexpected end of file"),
            Some(token) => match &token.kind {
                TokenKind::Integer(i) => AstNode::IntegerLiteral(*i, token.location.clone()),
                TokenKind::Float(f) => AstNode::FloatLiteral(*f, token.location.clone()),
                TokenKind::String(s) => {
                    AstNode::StringLiteral(s.to_string(), token.location.clone())
                }
                TokenKind::True => AstNode::BooleanLiteral(true, token.location.clone()),
                TokenKind::False => AstNode::BooleanLiteral(false, token.location.clone()),
                TokenKind::Identifier(s) => {
                    AstNode::Identifier(s.to_string(), token.location.clone())
                }
                TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Percent
                | TokenKind::Less
                | TokenKind::LessEqual
                | TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::EqualEqual
                | TokenKind::BangEqual
                | TokenKind::And
                | TokenKind::Or
                | TokenKind::Not
                | TokenKind::Bang => self.parse_operator(),
                TokenKind::Fun => self.parse_function_definition(),
                TokenKind::While => self.parse_while_expression(),
                TokenKind::If => self.parse_if_expression(),
                TokenKind::Call => self.parse_function_call(),
                _ => todo!("parse_node is not implemented for {:?} yet", token.kind),
            },
        }
    }

    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut ast = Vec::new();

        while !self.is_at_end() {
            let node = self.parse_node();
            ast.push(node);
        }

        ast
    }
}
