use std::collections::HashSet;

use crate::MiniDfError;
use crate::column::DataType;
use crate::error::Result;

pub struct Field {
    column_name: String,
    datatype: DataType,
    nullable: bool,
}

impl Field {
    pub fn new(column_name: &str, datatype: DataType, nullable: bool) -> Self {
        Field {
            column_name: column_name.to_string(),
            datatype,
            nullable,
        }
    }

    pub fn dtype(&self) -> DataType {
        self.datatype
    }

    pub fn nullable(&self) -> bool {
        self.nullable
    }
}

fn unique_fields(fields: &Vec<Field>) -> bool {
    let mut field_set = HashSet::new();

    for field in fields {
        if field_set.contains(&field.column_name) {
            return false;
        }

        field_set.insert(field.column_name.clone());
    }

    true
}

pub struct Schema {
    fields: Vec<Field>,
}

impl Schema {
    pub fn new(fields: Vec<Field>) -> Result<Self> {
        match unique_fields(&fields) {
            true => Ok(Schema { fields }),
            false => Err(MiniDfError::InvalidSchema),
        }
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn n_fields(&self) -> usize {
        self.fields.len()
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_schema_is_valid() {
        let fields = vec![];
        assert!(unique_fields(&fields))
    }

    #[test]
    fn single_field_schema_is_valid() {
        let fields = vec![Field::new("col_1", DataType::Bool, false)];
        assert!(unique_fields(&fields))
    }

    #[test]
    fn multi_field_valid_schema_is_valid() {
        let fields = vec![
            Field::new("col_1", DataType::Bool, false),
            Field::new("col_2", DataType::Int, false),
        ];
        assert!(unique_fields(&fields))
    }

    #[test]
    fn invalid_schema_is_invalid() {
        let fields = vec![
            Field::new("col_1", DataType::Bool, false),
            Field::new("col_1", DataType::Int, false),
        ];
        assert!(!unique_fields(&fields))
    }
}
