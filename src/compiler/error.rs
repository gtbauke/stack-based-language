#[derive(Debug, Clone)]
pub enum CompilerError {
    UnknownFunction(String),
}

#[derive(Debug, Clone)]
pub enum ResolverError {}
