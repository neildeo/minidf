//! Dataframe construction and inspection.
//!
//! A dataframe pairs a schema with an ordered collection of nameless columns.
//! The schema fields and columns are matched by position.

use std::{fmt::Display, iter::zip};

use crate::{MiniDfError, Result, column::Column, schema::Schema};

/// A typed, columnar dataframe.
///
/// A dataframe owns a schema and a collection of columns. The relationship
/// between them is positional: `schema.fields()[i]` describes `columns[i]`.
///
/// Dataframe construction enforces the core invariants connecting schema
/// metadata to physical column storage.
#[derive(Debug, PartialEq)]
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

    /// Returns a new dataframe containing the first `n` rows.
    ///
    /// If `n` is greater than the dataframe height, all rows are returned.
    /// If `n` is zero, a zero-row dataframe with the same schema is returned.
    ///
    /// This method preserves:
    ///
    /// - column order
    /// - row order
    /// - schema metadata
    /// - null values
    ///
    /// The returned dataframe owns its data, cloned from the original dataframe.
    pub fn head(&self, n: usize) -> Self {
        let out_schema = self.schema().clone();

        let effective_n = n.min(self.height());

        let out_columns: Vec<Column> = self
            .columns
            .iter()
            .map(|c| c.take_rows(0, effective_n))
            .collect();

        DataFrame::new(out_schema, out_columns).expect("Subset of valid dataframe should be valid")
    }

    /// Returns a new dataframe containing the last `n` rows.
    ///
    /// If `n` is greater than the dataframe height, all rows are returned.
    /// If `n` is zero, a zero-row dataframe with the same schema is returned.
    ///
    /// This method preserves:
    ///
    /// - column order
    /// - row order
    /// - schema metadata
    /// - null values
    ///
    /// The returned dataframe owns its data, cloned from the original dataframe.
    pub fn tail(&self, n: usize) -> Self {
        let out_schema = self.schema().clone();

        let effective_n = n.min(self.height());

        let out_columns: Vec<Column> = self
            .columns
            .iter()
            .map(|c| c.take_rows(self.height() - effective_n, effective_n))
            .collect();

        DataFrame::new(out_schema, out_columns).expect("Subset of valid dataframe should be valid")
    }

    pub fn select(&self, columns: impl IntoIterator<Item = impl AsRef<str>>) -> Result<DataFrame> {
        todo!()
    }
}

impl Display for DataFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h = self.height();
        let w = self.width();
        writeln!(f, "shape: ({}, {})", h, w)?;

        // Headers
        if !self.schema().is_empty() {
            writeln!(
                f,
                "{}",
                self.schema()
                    .fields()
                    .iter()
                    .fold("".to_string(), |acc, x| { acc + &x.to_string() + " | " })
                    .trim_end_matches(" | ") // Trim off final separator
            )?;
        }

        // Data
        for i in 0..h {
            writeln!(
                f,
                "{}",
                self.columns
                    .iter()
                    .fold("".to_string(), |acc, c| {
                        acc + c.get_formatted_value(i).as_str() + " | "
                    })
                    .trim_end_matches(" | ") // Trim off final separator
            )?;
        }

        Ok(())
    }
}
