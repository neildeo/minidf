//! Column storage and column-level inspection.
//!
//! Columns are the physical storage layer of the dataframe.
//! They are typed, nullable, and nameless.
//!
//! Column names and declared nullability live in the schema, not in the
//! column itself.

/// Logical data types supported by the dataframe.
///
/// Each `DataType` corresponds to one typed column representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Int,
    Float,
    Bool,
    String,
}

/// A typed, nullable column of dataframe values.
///
/// Columns store data but do not store names. A column's name, declared
/// data type, and declared nullability are described by the corresponding
/// schema field.
///
/// Null values are represented using `Option<T>` within each column variant.
#[derive(Debug, PartialEq)]
pub enum Column {
    Int(Vec<Option<i64>>),
    Float(Vec<Option<f64>>),
    Bool(Vec<Option<bool>>),
    String(Vec<Option<String>>),
}

impl Column {
    /// Constructs a non-null integer column.
    ///
    /// Each input value is wrapped as `Some(value)` internally.
    pub fn int(data: Vec<i64>) -> Self {
        Column::Int(data.into_iter().map(Some).collect())
    }

    /// Constructs a non-null floating-point column.
    ///
    /// Each input value is wrapped as `Some(value)` internally.
    pub fn float(data: Vec<f64>) -> Self {
        Column::Float(data.into_iter().map(Some).collect())
    }

    /// Constructs a non-null boolean column.
    ///
    /// Each input value is wrapped as `Some(value)` internally.
    pub fn bool(data: Vec<bool>) -> Self {
        Column::Bool(data.into_iter().map(Some).collect())
    }

    /// Constructs a non-null string column.
    ///
    /// Each input value is wrapped as `Some(value)` internally.
    pub fn string(data: Vec<String>) -> Self {
        Column::String(data.into_iter().map(Some).collect())
    }

    /// Constructs a nullable integer column.
    ///
    /// `None` values represent nulls.
    pub fn int_nullable(data: Vec<Option<i64>>) -> Self {
        Column::Int(data)
    }

    /// Constructs a nullable floating-point column.
    ///
    /// `None` values represent nulls.
    pub fn float_nullable(data: Vec<Option<f64>>) -> Self {
        Column::Float(data)
    }

    /// Constructs a nullable boolean column.
    ///
    /// `None` values represent nulls.
    pub fn bool_nullable(data: Vec<Option<bool>>) -> Self {
        Column::Bool(data)
    }

    /// Constructs a nullable string column.
    ///
    /// `None` values represent nulls.
    pub fn string_nullable(data: Vec<Option<String>>) -> Self {
        Column::String(data)
    }

    /// Returns the logical data type of the column.
    ///
    /// The data type is derived from the column variant rather than stored as
    /// separate metadata.
    pub fn dtype(&self) -> DataType {
        match self {
            Column::Int(_) => DataType::Int,
            Column::Float(_) => DataType::Float,
            Column::Bool(_) => DataType::Bool,
            Column::String(_) => DataType::String,
        }
    }

    /// Returns the number of values in the column.
    ///
    /// This corresponds to the number of dataframe rows represented by this
    /// column.
    pub fn len(&self) -> usize {
        match self {
            Column::Int(items) => items.len(),
            Column::Float(items) => items.len(),
            Column::Bool(items) => items.len(),
            Column::String(items) => items.len(),
        }
    }

    /// Returns `true` if the column is empty, i.e. contains no data.
    pub fn is_empty(&self) -> bool {
        match self {
            Column::Int(items) => items.is_empty(),
            Column::Float(items) => items.is_empty(),
            Column::Bool(items) => items.is_empty(),
            Column::String(items) => items.is_empty(),
        }
    }

    /// Returns `true` if the column contains at least one null value.
    ///
    /// This is a yes/no null check and may short-circuit. Use
    /// [`Column::null_count`] when the number of null values is required.
    pub fn has_nulls(&self) -> bool {
        match self {
            Column::Int(items) => items.iter().any(|x| x.is_none()),
            Column::Float(items) => items.iter().any(|x| x.is_none()),
            Column::Bool(items) => items.iter().any(|x| x.is_none()),
            Column::String(items) => items.iter().any(|x| x.is_none()),
        }
    }

    /// Returns the number of null values in the column.
    ///
    /// This inspects the full column.
    pub fn null_count(&self) -> usize {
        match self {
            Column::Int(items) => items.iter().filter(|x| x.is_none()).count(),
            Column::Float(items) => items.iter().filter(|x| x.is_none()).count(),
            Column::Bool(items) => items.iter().filter(|x| x.is_none()).count(),
            Column::String(items) => items.iter().filter(|x| x.is_none()).count(),
        }
    }

    /// Returns a new column containing a contiguous subset of rows.
    ///
    /// The returned column owns its data and preserves:
    ///
    /// - data type
    /// - value order
    /// - null values and their positions
    ///
    /// This is a low-level, crate-internal primitive used by higher-level
    /// dataframe operations such as `head` and `tail`.
    ///
    /// # Parameters
    ///
    /// - `start`: the starting row index (inclusive)
    /// - `n`: the number of rows to take
    ///
    /// Conceptually, this returns the column slice:
    ///
    /// ```text
    /// self[start .. start + n]
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics if the requested slice is out of bounds, i.e. if:
    ///
    /// - `start > self.len()`, or
    /// - `start + n > self.len()`
    ///
    /// Callers are expected to validate or clamp indices before calling this
    /// method. In particular, public dataframe APIs such as `head` and `tail`
    /// must ensure that only valid slice ranges are passed.
    pub(crate) fn take_rows(&self, start: usize, n: usize) -> Self {
        match self {
            Column::Int(items) => Column::int_nullable(items[start..start + n].to_vec()),
            Column::Float(items) => Column::float_nullable(items[start..start + n].to_vec()),
            Column::Bool(items) => Column::bool_nullable(items[start..start + n].to_vec()),
            Column::String(items) => Column::string_nullable(items[start..start + n].to_vec()),
        }
    }

    pub(crate) fn get_formatted_value(&self, index: usize) -> String {
        match self {
            Column::Int(items) => items[index].map_or("null".to_string(), |x| x.to_string()),
            Column::Float(items) => items[index].map_or("null".to_string(), |x| x.to_string()),
            Column::Bool(items) => items[index].map_or("null".to_string(), |x| x.to_string()),
            Column::String(items) => items[index]
                .clone()
                .map_or("null".to_string(), |x| x.to_string()),
        }
    }
}
