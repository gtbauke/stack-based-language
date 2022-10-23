use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INSERT_FILE_HERE:{}:{}", self.line, self.column)
    }
}

impl Location {
    pub fn combine(&self, other: &Self) -> Self {
        Self {
            line: self.line.min(other.line),
            column: self.column.min(other.column),
        }
    }
}
