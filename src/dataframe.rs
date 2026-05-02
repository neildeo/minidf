use crate::{column::Column, schema::Schema};

pub struct DataFrame {
    schema: Schema,
    columns: Vec<Column>,
}
