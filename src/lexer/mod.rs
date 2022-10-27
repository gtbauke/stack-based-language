use crate::token::{location::Location, Token, TokenKind};

const RESERVED_CHARS: [u8; 7] = [b'(', b')', b'[', b']', b'{', b'}', b'.'];

#[derive(Debug)]
pub struct Lexer {
    start: usize,
    current: usize,

    line: usize,
    column: usize,

    source: String,
}

// TODO: Add support for comments
// TODO: Add support for binary literals, octal literals, and hex literals
// TODO: Add support for escape sequences in strings
// TODO: Add error reporting mechanism
impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            source: source.to_string(),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn current_location(&self) -> Location {
        Location {
            line: self.line,
            column: self.column - (self.current - self.start),
        }
    }

    fn peek(&self, offset: usize) -> Option<u8> {
        let index = self.current + offset;

        if index >= self.source.len() {
            None
        } else {
            Some(self.source.as_bytes()[index])
        }
    }

    fn advance(&mut self) -> Option<u8> {
        self.current += 1;
        self.column += 1;

        self.source.as_bytes().get(self.current - 1).copied()
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek(0);

            match c {
                Some(b' ') | Some(b'\r') | Some(b'\t') => {
                    self.advance();
                }
                Some(b'\n') => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn new_token(&self, kind: TokenKind) -> Token {
        Token::new(kind, self.current_location())
    }

    fn handle_numbers(&mut self) -> Token {
        while let Some(c) = self.peek(0) {
            if !c.is_ascii_digit() {
                break;
            }

            self.advance();
        }

        if let Some(b'.') = self.peek(0) {
            self.advance();

            while let Some(c) = self.peek(0) {
                if !c.is_ascii_digit() {
                    break;
                }

                self.advance();
            }
        }

        let lexeme = &self.source[self.start..self.current];
        let kind = if lexeme.contains(".") {
            let value = lexeme.parse::<f64>();

            match value {
                Ok(value) => TokenKind::Float(value),
                Err(_) => TokenKind::Error,
            }
        } else {
            let value = lexeme.parse::<i64>();

            match value {
                Ok(value) => TokenKind::Integer(value),
                Err(_) => TokenKind::Error,
            }
        };

        self.new_token(kind)
    }

    fn handle_identifiers(&mut self) -> Token {
        while let Some(c) = self.peek(0) {
            if c.is_ascii_whitespace() || RESERVED_CHARS.contains(&c) {
                break;
            }

            self.advance();
        }

        let lexeme = &self.source[self.start..self.current];
        let kind = match lexeme {
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "elif" => TokenKind::Elif,
            "end" => TokenKind::End,
            "then" => TokenKind::Then,
            "while" => TokenKind::While,
            "in" => TokenKind::In,
            "let" => TokenKind::Let,
            "fun" => TokenKind::Fun,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "call" => TokenKind::Call,
            _ => TokenKind::Identifier(lexeme.to_string()),
        };

        self.new_token(kind)
    }

    fn handle_strings(&mut self) -> Token {
        while let Some(c) = self.peek(0) {
            if c == b'"' {
                break;
            }

            self.advance();
        }

        if self.peek(0) == Some(b'"') {
            self.advance();
        } else {
            return self.new_token(TokenKind::Error);
        }

        let lexeme = &self.source[self.start + 1..self.current - 1];
        self.new_token(TokenKind::String(lexeme.to_string()))
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;
        let c = self.advance();

        match c {
            Some(c) => match c {
                b'(' => self.new_token(TokenKind::OpenParen),
                b')' => self.new_token(TokenKind::CloseParen),
                b'[' => self.new_token(TokenKind::OpenBracket),
                b']' => self.new_token(TokenKind::CloseBracket),
                b'{' => self.new_token(TokenKind::OpenCurly),
                b'}' => self.new_token(TokenKind::CloseCurly),
                b',' => self.new_token(TokenKind::Comma),
                b';' => self.new_token(TokenKind::Semicolon),
                b'+' => self.new_token(TokenKind::Plus),
                b'-' => match self.peek(0) {
                    Some(b'>') => {
                        self.advance();
                        self.new_token(TokenKind::Arrow)
                    }
                    Some(c) if c.is_ascii_digit() => self.handle_numbers(),
                    _ => self.new_token(TokenKind::Minus),
                },
                b'*' => self.new_token(TokenKind::Star),
                b'/' => self.new_token(TokenKind::Slash),
                b'%' => self.new_token(TokenKind::Percent),
                b'^' => self.new_token(TokenKind::Caret),
                b'.' => match self.peek(0) {
                    Some(b'.') => {
                        self.advance();
                        self.new_token(TokenKind::DotDot)
                    }
                    _ => self.new_token(TokenKind::Dot),
                },
                b':' => match self.peek(0) {
                    Some(b':') => {
                        self.advance();
                        self.new_token(TokenKind::ColonColon)
                    }
                    _ => self.new_token(TokenKind::Colon),
                },
                b'!' => match self.peek(0) {
                    Some(b'=') => {
                        self.advance();
                        self.new_token(TokenKind::BangEqual)
                    }
                    _ => self.new_token(TokenKind::Bang),
                },
                b'=' => match self.peek(0) {
                    Some(b'=') => {
                        self.advance();
                        self.new_token(TokenKind::EqualEqual)
                    }
                    _ => self.new_token(TokenKind::Equal),
                },
                b'>' => match self.peek(0) {
                    Some(b'=') => {
                        self.advance();
                        self.new_token(TokenKind::GreaterEqual)
                    }
                    _ => self.new_token(TokenKind::Greater),
                },
                b'<' => match self.peek(0) {
                    Some(b'=') => {
                        self.advance();
                        self.new_token(TokenKind::LessEqual)
                    }
                    _ => self.new_token(TokenKind::Less),
                },
                b'"' => self.handle_strings(),
                _ if c.is_ascii_digit() => self.handle_numbers(),
                _ => self.handle_identifiers(),
            },
            None => self.new_token(TokenKind::EOF),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next();
            tokens.push(token.clone());

            if token.kind == TokenKind::EOF {
                break;
            }
        }

        tokens
    }
}
