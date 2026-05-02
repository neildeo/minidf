pub enum DataType {
    Int,
    Float,
    Bool,
    String,
}

pub enum Column {
    Int(Vec<Option<i64>>),
    Float(Vec<Option<f64>>),
    Bool(Vec<Option<bool>>),
    String(Vec<Option<String>>),
}

impl Column {
    pub fn new() -> Self {
        todo!("Constructor for a column");
    }

    pub fn dtype(&self) -> DataType {
        todo!("Return column's data type");
    }

    pub fn len(&self) -> usize {
        todo!("Return column length");
    }

    pub fn null_count(&self) -> usize {
        todo!("Return count of nulls in column");
    }
}
