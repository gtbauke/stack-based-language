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
    pub fn instruction_bytes(&self) -> &[u8; 4] {
        match self {
            Instruction::NoOp => &[0x00, 0x00, 0x00, 0x00],
            Instruction::LoadI64(_) => &[0x01, 0x00, 0x00, 0x00],
            Instruction::LoadF64(_) => &[0x02, 0x00, 0x00, 0x00],
            Instruction::LoadStr(_) => &[0x03, 0x00, 0x00, 0x00],
            Instruction::LoadBool(_) => &[0x04, 0x00, 0x00, 0x00],
            Instruction::LoadConstant(_) => &[0x05, 0x00, 0x00, 0x00],

            Instruction::Get(_) => &[0x06, 0x00, 0x00, 0x00],
            Instruction::Call(_) => &[0x07, 0x00, 0x00, 0x00],

            Instruction::Jump(_) => &[0x08, 0x00, 0x00, 0x00],
            Instruction::JumpIfFalse(_) => &[0x09, 0x00, 0x00, 0x00],
            Instruction::JumpIfTrue(_) => &[0x0A, 0x00, 0x00, 0x00],
            _ => todo!(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Instruction::NoOp => self.instruction_bytes().to_vec(),
            Instruction::LoadI64(i) => {
                let mut bytes = self.instruction_bytes().to_vec();
                bytes.extend_from_slice(&i.to_le_bytes());
                bytes
            }
            Instruction::LoadF64(f) => {
                let mut bytes = self.instruction_bytes().to_vec();
                bytes.extend_from_slice(&f.to_le_bytes());
                bytes
            }
            Instruction::LoadBool(b) => {
                let mut bytes = self.instruction_bytes().to_vec();
                bytes.extend_from_slice(&[*b as u8]);
                bytes
            }
            Instruction::LoadStr(id) => {
                let mut bytes = self.instruction_bytes().to_vec();
                bytes.extend_from_slice(&id.to_le_bytes());
                bytes
            }
            _ => todo!("to_bytes is not implemented for {:?} yet", self),
        }
    }
}
