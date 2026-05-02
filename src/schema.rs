use crate::column::DataType;

pub struct SchemaField {
    column_name: String,
    datatype: DataType,
    nullable: bool,
}

pub struct Schema {
    fields: Vec<SchemaField>,
}
