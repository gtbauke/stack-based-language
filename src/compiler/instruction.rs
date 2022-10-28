use crate::token::location::Location;

#[derive(Debug, Clone)]
pub enum InstructionKind {
    NoOp,
    Patch,

    LoadI64(i64),
    LoadF64(f64),
    LoadBool(bool),
    LoadConstant(usize),

    Get(String),
    Call(usize),

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Neg,
    Not,
    And,
    Or,
    Equals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Dup,
    Drop,
    Swap,
    Over,
    Print,

    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),

    DebugStack,

    Return,
    Halt,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub location: Location,
}

impl Instruction {
    pub fn new(kind: InstructionKind, location: &Location) -> Self {
        Self {
            kind,
            location: location.clone(),
        }
    }
}
