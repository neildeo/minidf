//! Schema representation.
//!
//! A schema is an ordered collection of fields describing the columns of a
//! dataframe.
//!
//! The schema owns metadata such as names, declared data types, and declared
//! nullability. It does not own column data.

use std::collections::HashSet;
use std::fmt::Display;

use crate::MiniDfError;
use crate::column::DataType;
use crate::error::Result;

/// Metadata describing one dataframe column.
///
/// A field contains the column name, declared data type, and declared
/// nullability. It describes a column by position within a dataframe schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    column_name: String,
    datatype: DataType,
    nullable: bool,
}

impl Field {
    /// Constructs a new field definition.
    pub fn new(column_name: &str, datatype: DataType, nullable: bool) -> Self {
        Field {
            column_name: column_name.to_string(),
            datatype,
            nullable,
        }
    }

    /// Returns the field name
    pub fn name(&self) -> &str {
        &self.column_name
    }

    /// Returns the declared data type of the field.
    pub fn dtype(&self) -> DataType {
        self.datatype
    }

    /// Returns `true` if the field permits null values.
    pub fn nullable(&self) -> bool {
        self.nullable
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.name())?;
        let type_str = match self.dtype() {
            DataType::Int => "Int",
            DataType::Float => "Float",
            DataType::Bool => "Bool",
            DataType::String => "String",
        };
        write!(f, "{type_str}")?;
        let nullable_str = if self.nullable() { "?" } else { "!" };
        write!(f, "{nullable_str}")
    }
}

fn unique_fields(fields: &Vec<Field>) -> Result<()> {
    let mut field_set = HashSet::new();

    for field in fields {
        if field_set.contains(&field.column_name) {
            return Err(MiniDfError::DuplicateColumnName {
                duplicate_name: field.column_name.clone(),
            });
        }

        field_set.insert(field.column_name.clone());
    }

    Ok(())
}

/// An ordered collection of dataframe fields.
///
/// The order of fields defines the order of dataframe columns. Field names
/// must be unique.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    fields: Vec<Field>,
}

impl Schema {
    /// Constructs a schema from an ordered collection of fields.
    ///
    /// Returns an error if the schema violates schema-level invariants, such as
    /// duplicate field names.
    pub fn new(fields: Vec<Field>) -> Result<Self> {
        unique_fields(&fields)?;
        Ok(Schema { fields })
    }

    /// Returns the number of fields in the schema.
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns `true` if the schema contains no fields
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns the ordered fields in the schema.
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    /// Return the field with the requested column name, together with its position.
    ///
    /// This performs a name-based lookup over the schema fields and returns a
    /// shared reference to the matching [`Field`] along with its zero-based index
    /// in the schema. Field names are expected to be unique within a valid schema,
    /// so at most one field should match.
    ///
    /// The returned index can be used to locate the corresponding column in a
    /// dataframe's column storage, since columns do not themselves store names.
    ///
    /// # Errors
    ///
    /// Returns [`MiniDfError::ColumnNotFound`] if no field exists with the
    /// requested name.
    pub fn get_field(&self, name: &str) -> Result<(&Field, usize)> {
        for (i, f) in self.fields().iter().enumerate() {
            if f.name() == name {
                return Ok((f, i));
            }
        }

        Err(MiniDfError::ColumnNotFound {
            name: name.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_schema_is_valid() {
        let fields = vec![];
        assert!(unique_fields(&fields).is_ok())
    }

    #[test]
    fn single_field_schema_is_valid() {
        let fields = vec![Field::new("col_1", DataType::Bool, false)];
        assert!(unique_fields(&fields).is_ok())
    }

    #[test]
    fn multi_field_valid_schema_is_valid() {
        let fields = vec![
            Field::new("col_1", DataType::Bool, false),
            Field::new("col_2", DataType::Int, false),
        ];
        assert!(unique_fields(&fields).is_ok())
    }

    #[test]
    fn invalid_schema_is_invalid() {
        let fields = vec![
            Field::new("col_1", DataType::Bool, false),
            Field::new("col_1", DataType::Int, false),
        ];
        assert!(unique_fields(&fields).is_err_and(|e| {
            matches!(
                e,
                MiniDfError::DuplicateColumnName {
                    duplicate_name
                } if duplicate_name == "col_1"
            )
        }))
    }
}
