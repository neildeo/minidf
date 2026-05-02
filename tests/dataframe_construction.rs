use minidf::{Column, DataFrame, DataType, Field, MiniDfError, Schema};

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

#[test]
fn errors_on_mismatched_schema_and_columns_length() {
    let schema = Schema::new(vec![
        Field::new("col_1", DataType::Int, false),
        Field::new("col_2", DataType::String, false),
    ])
    .expect("Schema should be valid");

    let col_1: Vec<i64> = vec![4, 7];

    let maybe_df = DataFrame::new(schema, vec![Column::int(col_1)]);

    assert!(maybe_df.is_err_and(|e| e == MiniDfError::SchemaMismatch))
}

#[test]
fn errors_on_mismatched_schema_and_columns_dtypes() {
    let schema = Schema::new(vec![
        Field::new("col_1", DataType::Int, false),
        Field::new("col_2", DataType::String, false),
    ])
    .expect("Schema should be valid");

    let col_1: Vec<f64> = vec![4., 7.];
    let col_2: Vec<String> = vec!["hello".to_string(), "world".to_string()];

    let maybe_df = DataFrame::new(schema, vec![Column::float(col_1), Column::string(col_2)]);

    assert!(maybe_df.is_err_and(|e| e == MiniDfError::SchemaMismatch))
}

#[test]
fn errors_on_unequal_column_lengths() {
    let schema = Schema::new(vec![
        Field::new("col_1", DataType::Int, false),
        Field::new("col_2", DataType::String, false),
    ])
    .expect("Schema should be valid");

    let col_1: Vec<f64> = vec![4., 7., 19.];
    let col_2: Vec<String> = vec!["hello".to_string(), "world".to_string()];

    let maybe_df = DataFrame::new(schema, vec![Column::float(col_1), Column::string(col_2)]);

    assert!(maybe_df.is_err_and(|e| e == MiniDfError::SchemaMismatch))
}

#[test]
fn errors_duplicate_column_names() {
    let schema = Schema::new(vec![
        Field::new("col_1", DataType::Int, false),
        Field::new("col_1", DataType::String, false),
    ]);

    assert!(schema.is_err_and(|e| e == MiniDfError::InvalidSchema))
}

#[test]
fn errors_on_null_in_non_nullable_field() {
    let schema = Schema::new(vec![
        Field::new("col_1", DataType::Int, true),
        Field::new("col_2", DataType::String, false),
    ])
    .expect("Schema should be valid");

    let col_1: Vec<Option<f64>> = vec![None, Some(2.)];
    let col_2: Vec<Option<bool>> = vec![Some(true), None];

    let maybe_df = DataFrame::new(
        schema,
        vec![Column::float_nullable(col_1), Column::bool_nullable(col_2)],
    );

    assert!(maybe_df.is_err_and(|e| e == MiniDfError::SchemaMismatch))
}
