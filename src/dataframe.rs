use std::iter::zip;

use crate::{MiniDfError, Result, column::Column, schema::Schema};

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
    if schema.n_fields() != columns.len() {
        Err(MiniDfError::FieldColumnCountMismatch {
            fields: schema.n_fields(),
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
        if !f.nullable() && c.contains_null() {
            return Err(MiniDfError::NullabilityViolation {
                field_name: f.name().to_string(),
            });
        }
    }

    Ok(())
}

impl DataFrame {
    pub fn new(schema: Schema, columns: Vec<Column>) -> Result<Self> {
        columns_have_equal_length(&columns)?;
        schema_length_equals_number_of_columns(&schema, &columns)?;
        schema_and_columns_datatypes_match(&schema, &columns)?;
        non_null_columns_have_no_nulls(&schema, &columns)?;

        Ok(DataFrame { schema, columns })
    }

    pub fn is_empty(&self) -> bool {
        self.schema.is_empty()
    }

    pub fn height(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        self.columns[0].len()
    }

    pub fn width(&self) -> usize {
        self.schema.len()
    }
}
