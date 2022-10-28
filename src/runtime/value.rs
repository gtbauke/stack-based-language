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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::I64(i) => write!(f, "{}", i),
            Value::F64(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::RawString(s) => write!(f, "{}", s),
        }
    }
}

// TODO: change this to be a pointer to a string in the heap
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

impl std::fmt::Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "str pointer to {}", self.string_index)
    }
}
