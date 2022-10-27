use self::location::Location;
use std::fmt::Display;

pub mod location;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,
    Comma,
    Dot,
    Colon,
    ColonColon,
    DotDot,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Bang,
    Equal,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Arrow,

    If,
    Else,
    Elif,
    End,
    Then,
    While,
    In,
    Let,
    Fun,
    And,
    Or,
    Not,
    True,
    False,
    Call,

    Identifier(String),
    String(String),
    Integer(i64),
    Float(f64),

    Error,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub location: Location,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} at {}", self.kind, self.location)
    }
}

impl Token {
    pub fn new(kind: TokenKind, location: Location) -> Self {
        Self { kind, location }
    }
}
