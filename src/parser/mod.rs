use crate::{
    lexer::Lexer,
    token::{Token, TokenKind},
};

use self::ast::{AstNode, Block};

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

    fn consume(&mut self, expected: TokenKind) -> Option<Token> {
        if self
            .current
            .as_ref()
            .map_or(false, |token| token.kind == expected)
        {
            self.advance();
            self.previous.clone()
        } else {
            None
        }
    }

    fn consume_identifier(&mut self) -> Option<String> {
        if let Some(Token {
            kind: TokenKind::Identifier(name),
            ..
        }) = self.current.clone()
        {
            self.advance();
            Some(name)
        } else {
            None
        }
    }

    fn consume_any_of(&mut self, kinds: Vec<TokenKind>) -> Option<Token> {
        if self
            .current
            .as_ref()
            .map_or(false, |token| kinds.contains(&token.kind))
        {
            self.advance();
            self.previous.clone()
        } else {
            None
        }
    }

    fn parse_block(&mut self) -> Block {
        let mut nodes = Vec::new();

        loop {
            match self.current.as_ref() {
                None => panic!("Unexpected end of file"),
                Some(token) => match token.kind {
                    TokenKind::End => break,
                    _ => {
                        nodes.push(self.parse_expression());
                        self.advance();
                    }
                },
            }
        }

        self.consume(TokenKind::End);

        Block { nodes }
    }

    fn parse_function_definition(&mut self) -> AstNode {
        let location = self.current.as_ref().unwrap().location.clone();
        self.advance();

        let name = match self.consume_identifier() {
            Some(name) => name,
            None => {
                // TODO: error handling
                panic!("Expected identifier after 'def' keyword");
            }
        };

        self.consume(TokenKind::In);

        let body = self.parse_block();
        let location =
            location.combine(body.nodes.last().map_or(&location, |node| node.location()));

        AstNode::FunctionDeclaration {
            name,
            body,
            location,
        }
    }

    fn parse_if_expression(&mut self) -> AstNode {
        let location = self.current.as_ref().unwrap().location.clone();
        self.advance();

        let mut then_branch = Vec::<AstNode>::new();
        let mut else_branch = Vec::<AstNode>::new();

        loop {
            match self.current.as_ref() {
                None => {
                    panic!("Unexpected end of file");
                }
                Some(token) => match token.kind {
                    TokenKind::End => break,
                    TokenKind::Else => break,
                    _ => {
                        then_branch.push(self.parse_expression());
                        self.advance();
                    }
                },
            }
        }

        let token = self.consume_any_of(vec![TokenKind::End, TokenKind::Else]);

        match token {
            None => panic!("Unexpected end of file"),
            Some(token) => match token.kind {
                TokenKind::Else => {
                    loop {
                        let token = self.current.as_ref();

                        match token {
                            None => {
                                panic!("Unexpected end of file");
                            }
                            Some(token) => match token.kind {
                                TokenKind::End => break,
                                _ => {
                                    else_branch.push(self.parse_expression());
                                    self.advance();
                                }
                            },
                        }
                    }

                    self.consume(TokenKind::End);
                }
                _ => {}
            },
        }

        let location = location.combine(&self.previous.as_ref().unwrap().location);

        AstNode::IfExpression {
            location,
            then_branch: Block { nodes: then_branch },
            else_branch: if else_branch.len() > 0 {
                Some(Block { nodes: else_branch })
            } else {
                None
            },
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
                    TokenKind::If => self.parse_if_expression(),
                    TokenKind::Fun => self.parse_function_definition(),
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
