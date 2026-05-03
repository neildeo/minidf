//! Dataframe construction and inspection.
//!
//! A dataframe pairs a schema with an ordered collection of nameless columns.
//! The schema fields and columns are matched by position.

use std::iter::zip;

use crate::{MiniDfError, Result, column::Column, schema::Schema};

/// A typed, columnar dataframe.
///
/// A dataframe owns a schema and a collection of columns. The relationship
/// between them is positional: `schema.fields()[i]` describes `columns[i]`.
///
/// Dataframe construction enforces the core invariants connecting schema
/// metadata to physical column storage.
#[derive(Debug)]
pub struct DataFrame {
    schema: Schema,
    columns: Vec<Column>,
}

fn columns_have_equal_length(columns: &[Column]) -> Result<()> {
    if columns.is_empty() {
        return Ok(());
    }

    let first_len = columns[0].len();

    for (i, c) in columns.iter().enumerate() {
        if c.len() != first_len {
            return Err(MiniDfError::ColumnLengthMismatch {
                column_index: i,
                expected: first_len,
                actual: c.len(),
            });
        }
    }

    Ok(())
}

fn schema_length_equals_number_of_columns(schema: &Schema, columns: &[Column]) -> Result<()> {
    if schema.len() != columns.len() {
        Err(MiniDfError::FieldColumnCountMismatch {
            fields: schema.len(),
            columns: columns.len(),
        })
    } else {
        Ok(())
    }
}

// Assumes equal lengths due to zip behaviour
fn schema_and_columns_datatypes_match(schema: &Schema, columns: &[Column]) -> Result<()> {
    for (f, c) in zip(schema.fields(), columns) {
        if f.dtype() != c.dtype() {
            return Err(MiniDfError::DatatypeMismatch {
                field_name: f.name().to_string(),
                expected: f.dtype(),
                actual: c.dtype(),
            });
        }
    }

    Ok(())
}

// Assumes equal lengths due to zip behaviour
fn non_null_columns_have_no_nulls(schema: &Schema, columns: &[Column]) -> Result<()> {
    for (f, c) in zip(schema.fields(), columns) {
        if !f.nullable() && c.has_nulls() {
            return Err(MiniDfError::NullabilityViolation {
                field_name: f.name().to_string(),
            });
        }
    }

    Ok(())
}

impl DataFrame {
    /// Constructs a dataframe from a schema and ordered columns.
    ///
    /// Construction validates that:
    ///
    /// - the number of fields matches the number of columns
    /// - each field's declared data type matches the corresponding column
    /// - non-nullable fields do not correspond to columns containing nulls
    /// - all columns have equal length
    pub fn new(schema: Schema, columns: Vec<Column>) -> Result<Self> {
        columns_have_equal_length(&columns)?;
        schema_length_equals_number_of_columns(&schema, &columns)?;
        schema_and_columns_datatypes_match(&schema, &columns)?;
        non_null_columns_have_no_nulls(&schema, &columns)?;

        Ok(DataFrame { schema, columns })
    }

    /// Returns the dataframe's schema.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// Returns the number of rows in the dataframe.
    ///
    /// The height is derived from column length. A dataframe with no columns
    /// has height zero.
    pub fn height(&self) -> usize {
        if self.width() == 0 {
            return 0;
        }

        self.columns[0].len()
    }

    /// Returns the number of columns in the dataframe.
    pub fn width(&self) -> usize {
        self.schema.len()
    }
}
