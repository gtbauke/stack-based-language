#[derive(Debug, Clone)]
pub enum Value {
    I64(i64),
    F64(f64),
    Bool(bool),
    String(Str),
    RawString(String),
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
