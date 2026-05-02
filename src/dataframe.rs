use crate::{Result, column::Column, schema::Schema};

pub struct DataFrame {
    schema: Schema,
    columns: Vec<Column>,
}

impl DataFrame {
    pub fn new(schema: Schema, colums: Vec<Column>) -> Result<Self> {
        todo!()
    }

    pub fn height(&self) -> usize {
        todo!()
    }

    pub fn width(&self) -> usize {
        todo!()
    }
}
