//! Error types used by the crate.

use std::{error::Error, fmt::Display};

use crate::DataType;

/// Error type for dataframe construction and validation failures.
#[derive(Debug, PartialEq)]
pub enum MiniDfError {
    DuplicateColumnName {
        duplicate_name: String,
    },
    FieldColumnCountMismatch {
        fields: usize,
        columns: usize,
    },
    DatatypeMismatch {
        field_name: String,
        expected: DataType,
        actual: DataType,
    },
    ColumnLengthMismatch {
        column_index: usize,
        expected: usize,
        actual: usize,
    },
    NullabilityViolation {
        field_name: String,
    },
}

impl Display for MiniDfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiniDfError::DuplicateColumnName { duplicate_name } => write!(
                f,
                "Schema has multiple fields with the same name: {}",
                duplicate_name
            ),
            MiniDfError::FieldColumnCountMismatch { fields, columns } => write!(
                f,
                "Schema has {fields} fields, but {columns} columns were provided"
            ),
            MiniDfError::DatatypeMismatch {
                field_name,
                expected,
                actual,
            } => write!(
                f,
                "Expected data type {expected:?} for {field_name}, but got {actual:?}"
            ),
            MiniDfError::ColumnLengthMismatch {
                column_index,
                expected,
                actual,
            } => write!(
                f,
                "Column with index {column_index} has the wrong length: expected {expected} but got {actual}"
            ),
            MiniDfError::NullabilityViolation { field_name } => write!(
                f,
                "Field {field_name} is declared as non-null but the provided column contains null values"
            ),
        }
    }
}

impl Error for MiniDfError {}

/// Crate-local result type using [`MinidfError`].
pub type Result<T> = std::result::Result<T, MiniDfError>;
