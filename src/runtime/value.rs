#[derive(Debug, Clone)]
pub enum Value {
    I64(i64),
    F64(f64),
    Bool(bool),
    String(Str),
    RawString(String),
}

impl Value {
    pub fn equals(&self, other: &Self) -> Self {
        match (self, other) {
            (Value::I64(a), Value::I64(b)) => Value::Bool(a == b),
            (Value::F64(a), Value::F64(b)) => Value::Bool(a == b),
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
            (Value::String(a), Value::String(b)) => Value::Bool(a.string_index == b.string_index),
            (Value::RawString(a), Value::RawString(b)) => Value::Bool(a == b),
            _ => Value::Bool(false),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Str {
    pub string_index: usize,
    pub length: usize,
}

impl Str {
    pub fn new(string_index: usize, length: usize) -> Self {
        Self {
            string_index,
            length,
        }
    }
}
