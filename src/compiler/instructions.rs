// TODO: think of a better layout for instructions, because argument sizes can vary

/*
Instruction layout

1 byte -> Instruction Type
1 byte -> Instruction Args Size
n bytes -> Instruction Args

*/

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Instruction {
    NoOp,

    LoadI64(i64),
    LoadF64(f64),
    LoadStr(usize),
    LoadBool(bool),
    LoadConstant(usize),

    Get(String),
    Call(usize),

    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
}

impl Instruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Instruction::NoOp => vec![0x00, 0x00, 0x00, 0x00],
            Instruction::LoadI64(i) => {
                let mut bytes = vec![0x01, 0x08, 0x01, 0x00];
                bytes.extend_from_slice(&i.to_le_bytes());
                bytes
            }
            Instruction::LoadF64(f) => {
                let mut bytes = vec![0x02, 0x08, 0x01, 0x00];
                bytes.extend_from_slice(&f.to_le_bytes());
                bytes
            }
            Instruction::LoadBool(b) => {
                let mut bytes = vec![0x04, 0x01, 0x01, 0x00];
                bytes.extend_from_slice(&[*b as u8]);
                bytes
            }
            Instruction::LoadStr(id) => {
                let mut bytes = vec![0x05, 0x04, 0x01, 0x00];
                bytes.extend_from_slice(&id.to_le_bytes());
                bytes
            }
            _ => todo!("to_bytes is not implemented for {:?} yet", self),
        }
    }
}
