use crate::token::location::Location;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    StackUnderflow(Location),
    InvalidTypes(Location),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::StackUnderflow(location) => write!(
                f,
                "Stack underflow at {}:{}",
                location.line, location.column
            ),
            RuntimeError::InvalidTypes(location) => {
                write!(f, "Invalid types at {}:{}", location.line, location.column)
            }
        }
    }
}
