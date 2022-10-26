#[derive(Debug, Clone)]
pub enum Instruction {
    NoOp,

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

    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
}
