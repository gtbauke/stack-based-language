#[derive(Debug, Clone)]
pub enum RuntimeError {
    StackUnderflow,
    InvalidTypes,
}
