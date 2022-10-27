use crate::compiler::{instruction::Instruction, program::Program};

use self::{
    error::RuntimeError,
    value::{Str, Value},
};

mod error;
pub mod value;

#[derive(Debug)]
pub struct Interpreter {
    stack: Vec<Value>,
    program: Program,
    bp: usize,
    ip: usize,

    brp: usize,
    irp: usize,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        let bp = &program.functions.get("main").map_or(0, |i| *i);

        Self {
            stack: Vec::new(),
            program,
            bp: *bp,
            ip: 0,

            brp: 0,
            irp: 0,
        }
    }

    fn current_instruction(&self) -> &Instruction {
        &self.program.blocks[self.bp].instructions[self.ip]
    }

    pub fn interpret(&mut self) -> Result<Value, RuntimeError> {
        loop {
            if self.bp >= self.program.blocks.len()
                || self.ip >= self.program.blocks[self.bp].instructions.len()
            {
                break;
            }

            let instruction = self.current_instruction().clone();

            match instruction {
                Instruction::Halt => break,
                Instruction::NoOp => {}
                Instruction::LoadI64(value) => {
                    self.stack.push(Value::I64(value));
                    self.ip += 1;
                }
                Instruction::LoadF64(value) => {
                    self.stack.push(Value::F64(value));
                    self.ip += 1;
                }
                Instruction::LoadBool(value) => {
                    self.stack.push(Value::Bool(value));
                    self.ip += 1;
                }
                Instruction::LoadConstant(index) => {
                    let string_length = self.program.strings[index].len();
                    let value = Value::String(Str::new(index, string_length));

                    self.stack.push(value);
                    self.ip += 1;
                }
                Instruction::Add => {
                    // TODO: handle types here
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => return Err(RuntimeError::StackUnderflow),
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => return Err(RuntimeError::StackUnderflow),
                    };

                    let result = match (left, right) {
                        (Value::I64(left), Value::I64(right)) => Value::I64(left + right),
                        (Value::F64(left), Value::F64(right)) => Value::F64(left + right),
                        (Value::I64(left), Value::F64(right)) => Value::F64(left as f64 + right),
                        (Value::F64(left), Value::I64(right)) => Value::F64(left + right as f64),
                        (Value::String(left), Value::String(right)) => {
                            let mut string = self.program.strings[left.string_index].clone();
                            string.push_str(&self.program.strings[right.string_index]);

                            let index = self.program.add_string(&string);
                            Value::String(Str::new(index, string.len()))
                        }
                        _ => return Err(RuntimeError::InvalidTypes),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                Instruction::Call(index) => {
                    self.brp = self.bp;
                    self.irp = self.ip + 1;

                    self.bp = index;
                    self.ip = 0;
                }
                Instruction::Return => {
                    self.bp = self.brp;
                    self.ip = self.irp;
                }
                _ => todo!("Instruction not implemented: {:?}", instruction),
            }
        }

        let result = match self.stack.pop() {
            Some(value) => match value {
                Value::String(string) => {
                    let string = self.program.strings[string.string_index].clone();
                    Value::RawString(string)
                }
                _ => value,
            },
            None => Value::I64(0),
        };

        Ok(result)
    }
}
