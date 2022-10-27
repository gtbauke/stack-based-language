use core::panic;

use crate::token::{Token, TokenKind};

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

    fn is_at_end(&self) -> bool {
        self.tokens[self.current].kind == TokenKind::EOF
    }

    fn advance(&mut self) -> Option<&Token> {
        self.current += 1;

        if self.is_at_end() {
            None
        } else {
            Some(&self.tokens[self.current - 1])
        }
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

    fn consume(&mut self, expected: TokenKind) -> Option<&Token> {
        match self.tokens.get(self.current) {
            Some(token) if token.kind == expected => {
                self.current += 1;
                Some(token)
            }
            _ => None,
        }
    }

    fn consume_identifier(&mut self) -> Option<String> {
        match self.tokens.get(self.current) {
            Some(Token {
                kind: TokenKind::Identifier(lexeme),
                ..
            }) => {
                self.current += 1;
                Some(lexeme.clone())
            }
            _ => panic!("Expected identifier"),
        }
    }

    fn consume_any_of(&mut self, kinds: Vec<TokenKind>) -> Option<&Token> {
        match self.tokens.get(self.current) {
            Some(token) if kinds.contains(&token.kind) => {
                self.current += 1;
                Some(token)
            }
            _ => panic!("Expected one of {:?}", kinds),
        }
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.current + offset)
    }

    fn parse_block(&mut self) -> Block {
        let mut nodes = Vec::new();

        loop {
            let token = self.peek(0);

            match token {
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
        let location = self.peek(0).unwrap().location.clone();
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
        let location = self.peek(0).unwrap().location.clone();
        self.advance();

        let mut then_branch = Vec::<AstNode>::new();
        let mut else_branch = Vec::<AstNode>::new();

        loop {
            let token = self.peek(0);

            match token {
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
                        let token = self.peek(0);

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

        let location =
            location.combine(else_branch.last().map_or(&location, |node| node.location()));

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

    fn parse_function_call(&mut self) -> AstNode {
        let location = self.peek(0).unwrap().location.clone();
        self.advance();

        let name = match self.consume_identifier() {
            Some(name) => name,
            None => {
                panic!("You need to specify a function to call")
            }
        };

        AstNode::FunctionCall { name, location }
    }

    fn parse_expression(&mut self) -> AstNode {
        let token = self.peek(0);

        match token {
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
                    TokenKind::Call => self.parse_function_call(),
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
            if self.is_at_end() {
                break;
            }

            nodes.push(self.parse_expression());
        }

        nodes
    }
}
