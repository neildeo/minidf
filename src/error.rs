use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum MiniDfError {
    SchemaMismatch,
}

impl Display for MiniDfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiniDfError::SchemaMismatch => write!(
                f,
                "The provided schema does not match the specified columns."
            ),
        }
    }
}

impl Error for MiniDfError {}

pub type Result<T> = std::result::Result<T, MiniDfError>;
