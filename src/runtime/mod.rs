use crate::compiler::{
    instruction::{Instruction, InstructionKind},
    program::Program,
};

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

            match instruction.kind {
                InstructionKind::Halt => break,
                InstructionKind::NoOp => {}
                InstructionKind::DebugStack => {
                    println!("Stack: {:#?}", self.stack);
                    self.ip += 1;
                }
                InstructionKind::LoadI64(value) => {
                    self.stack.push(Value::I64(value));
                    self.ip += 1;
                }
                InstructionKind::LoadF64(value) => {
                    self.stack.push(Value::F64(value));
                    self.ip += 1;
                }
                InstructionKind::LoadBool(value) => {
                    self.stack.push(Value::Bool(value));
                    self.ip += 1;
                }
                InstructionKind::LoadConstant(index) => {
                    let string_length = self.program.strings[index].len();
                    let value = Value::String(Str::new(index, string_length));

                    self.stack.push(value);
                    self.ip += 1;
                }
                InstructionKind::Add => {
                    // TODO: handle types here
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
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
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::Sub => {
                    // TODO: handle types here
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match (left, right) {
                        (Value::I64(left), Value::I64(right)) => Value::I64(left - right),
                        (Value::F64(left), Value::F64(right)) => Value::F64(left - right),
                        (Value::I64(left), Value::F64(right)) => Value::F64(left as f64 - right),
                        (Value::F64(left), Value::I64(right)) => Value::F64(left - right as f64),
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::Mul => {
                    // TODO: handle types here
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match (left, right) {
                        (Value::I64(left), Value::I64(right)) => Value::I64(left * right),
                        (Value::F64(left), Value::F64(right)) => Value::F64(left * right),
                        (Value::I64(left), Value::F64(right)) => Value::F64(left as f64 * right),
                        (Value::F64(left), Value::I64(right)) => Value::F64(left * right as f64),
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::Div => {
                    // TODO: handle types here
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match (left, right) {
                        (Value::I64(left), Value::I64(right)) => {
                            Value::F64(left as f64 / right as f64)
                        }
                        (Value::F64(left), Value::F64(right)) => Value::F64(left / right),
                        (Value::I64(left), Value::F64(right)) => Value::F64(left as f64 / right),
                        (Value::F64(left), Value::I64(right)) => Value::F64(left / right as f64),
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::Mod => {
                    // TODO: handle types here
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match (left, right) {
                        (Value::I64(left), Value::I64(right)) => Value::I64(left % right),
                        (Value::F64(left), Value::F64(right)) => Value::F64(left % right),
                        (Value::I64(left), Value::F64(right)) => Value::F64(left as f64 % right),
                        (Value::F64(left), Value::I64(right)) => Value::F64(left % right as f64),
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::Not => {
                    let value = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match value {
                        Value::Bool(value) => Value::Bool(!value),
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::And => {
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match (left, right) {
                        (Value::Bool(left), Value::Bool(right)) => Value::Bool(left && right),
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::Or => {
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match (left, right) {
                        (Value::Bool(left), Value::Bool(right)) => Value::Bool(left || right),
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::Equals => {
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = right.equals(&left);

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::NotEquals => {
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match right.equals(&left) {
                        Value::Bool(value) => Value::Bool(!value),
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::LessThan => {
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match (left, right) {
                        (Value::I64(left), Value::I64(right)) => Value::Bool(left < right),
                        (Value::F64(left), Value::F64(right)) => Value::Bool(left < right),
                        (Value::I64(left), Value::F64(right)) => Value::Bool((left as f64) < right),
                        (Value::F64(left), Value::I64(right)) => Value::Bool(left < right as f64),
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::LessThanEquals => {
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match (left, right) {
                        (Value::I64(left), Value::I64(right)) => Value::Bool(left <= right),
                        (Value::F64(left), Value::F64(right)) => Value::Bool(left <= right),
                        (Value::I64(left), Value::F64(right)) => {
                            Value::Bool((left as f64) <= right)
                        }
                        (Value::F64(left), Value::I64(right)) => Value::Bool(left <= right as f64),
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::GreaterThan => {
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match (left, right) {
                        (Value::I64(left), Value::I64(right)) => Value::Bool(left > right),
                        (Value::F64(left), Value::F64(right)) => Value::Bool(left > right),
                        (Value::I64(left), Value::F64(right)) => Value::Bool((left as f64) > right),
                        (Value::F64(left), Value::I64(right)) => Value::Bool(left > right as f64),
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::GreaterThanEquals => {
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match (left, right) {
                        (Value::I64(left), Value::I64(right)) => Value::Bool(left >= right),
                        (Value::F64(left), Value::F64(right)) => Value::Bool(left >= right),
                        (Value::I64(left), Value::F64(right)) => {
                            Value::Bool((left as f64) >= right)
                        }
                        (Value::F64(left), Value::I64(right)) => Value::Bool(left >= right as f64),
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    self.stack.push(result);
                    self.ip += 1;
                }
                InstructionKind::Call(index) => {
                    self.brp = self.bp;
                    self.irp = self.ip + 1;

                    self.bp = index;
                    self.ip = 0;
                }
                InstructionKind::JumpIfFalse(index) => {
                    let value = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let result = match value {
                        Value::Bool(value) => value,
                        _ => return Err(RuntimeError::InvalidTypes(instruction.location.clone())),
                    };

                    if result {
                        self.ip += 1;
                    } else {
                        self.ip = index;
                    }
                }
                InstructionKind::Jump(index) => self.ip = index,
                InstructionKind::Return => {
                    self.bp = self.brp;
                    self.ip = self.irp;
                }
                InstructionKind::Dup => {
                    let value = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    self.stack.push(value.clone());
                    self.stack.push(value);

                    self.ip += 1;
                }
                InstructionKind::Drop => match self.stack.pop() {
                    Some(_) => self.ip += 1,
                    None => return Err(RuntimeError::StackUnderflow(instruction.location.clone())),
                },
                InstructionKind::Swap => {
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    self.stack.push(right);
                    self.stack.push(left);

                    self.ip += 1;
                }
                InstructionKind::Over => {
                    let right = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    let left = match self.stack.pop() {
                        Some(value) => value,
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    self.stack.push(left.clone());
                    self.stack.push(right);
                    self.stack.push(left);

                    self.ip += 1;
                }
                InstructionKind::Print => {
                    let value = match self.stack.pop() {
                        Some(value) => match value {
                            Value::String(Str { string_index, .. }) => {
                                Value::RawString(self.program.strings[string_index].clone())
                            }
                            _ => value,
                        },
                        None => {
                            return Err(RuntimeError::StackUnderflow(instruction.location.clone()))
                        }
                    };

                    print!("{}", value);
                    self.ip += 1;
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
