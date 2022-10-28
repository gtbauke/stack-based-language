use crate::token::location::Location;

#[derive(Debug, Clone)]
pub enum CompilerError {
    UnknownFunction(String),
}

#[derive(Debug, Clone)]
pub enum ResolverError {
    CallToMain(Location),
}

impl std::fmt::Display for ResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolverError::CallToMain(location) => {
                write!(
                    f,
                    "Unexpected call to main at {}:{}",
                    location.line, location.column
                )
            }
        }
    }
}
