use crate::column::DataType;

pub struct Field {
    column_name: String,
    datatype: DataType,
    nullable: bool,
}

impl Field {
    pub fn new(column_name: &str, datatype: DataType, nullable: bool) -> Self {
        todo!()
    }
}

pub struct Schema {
    fields: Vec<Field>,
}

impl Schema {
    pub fn new(fields: Vec<Field>) -> Self {
        todo!()
    }
}
