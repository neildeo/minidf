use minidf::{Column, DataFrame, DataType, Field, Schema};

#[test]
fn constructs_dataframe_from_matching_schema_and_columns() {
    let schema = Schema::new(vec![
        Field::new("col_1", DataType::Int, false),
        Field::new("col_2", DataType::String, false),
    ])
    .expect("Schema should be valid");

    let col_1: Vec<i64> = vec![4, 7];
    let col_2: Vec<String> = vec!["hello".to_string(), "world".to_string()];

    let maybe_df = DataFrame::new(schema, vec![Column::int(col_1), Column::string(col_2)]);

    let df = maybe_df.expect("Dataframe construction should succeed");

    assert_eq!(df.height(), 2usize);
    assert_eq!(df.width(), 2usize);
}
