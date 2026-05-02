use std::iter::zip;

use crate::{MiniDfError, Result, column::Column, schema::Schema};

pub struct DataFrame {
    schema: Schema,
    columns: Vec<Column>,
}

fn validate_schema_and_columns(schema: &Schema, columns: &[Column]) -> bool {
    [
        columns_have_equal_length(columns),
        schema_length_equals_number_of_columns(schema, columns),
        schema_and_columns_datatypes_match(schema, columns),
        non_null_columns_have_no_nulls(schema, columns),
    ]
    .iter()
    .all(|x| *x)
}

fn columns_have_equal_length(columns: &[Column]) -> bool {
    if columns.is_empty() {
        return true;
    }

    let first_len = columns[0].len();

    columns.iter().all(|c| c.len() == first_len)
}

fn schema_length_equals_number_of_columns(schema: &Schema, columns: &[Column]) -> bool {
    schema.n_fields() == columns.len()
}

// Assumes equal lengths due to zip behaviour
fn schema_and_columns_datatypes_match(schema: &Schema, columns: &[Column]) -> bool {
    zip(schema.fields(), columns).all(|(f, c)| f.dtype() == c.dtype())
}

// Assumes equal lengths due to zip behaviour
fn non_null_columns_have_no_nulls(schema: &Schema, columns: &[Column]) -> bool {
    zip(schema.fields(), columns).all(|(f, c)| f.nullable() || !c.contains_null())
}

impl DataFrame {
    pub fn new(schema: Schema, columns: Vec<Column>) -> Result<Self> {
        match validate_schema_and_columns(&schema, &columns) {
            true => Ok(DataFrame { schema, columns }),
            false => Err(MiniDfError::SchemaMismatch),
        }
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
