use minidf::{Column, DataFrame, DataType, Field, Schema};

/// Returns a sample dataframe
///
/// Creates a small dataframe, useful for testing and validation.
///
/// The schema is as follows:
/// 1. "id" - Int - non-null
/// 2. "name" - String - nullable
/// 3. "is_important" - Bool - nullable
///
/// If `non_empty` is `true`, the produced dataframe has height 5, with some null values in the nullable columns. Otherwise the returned dataframe has 0 rows.
pub fn sample_dataframe(non_empty: bool) -> DataFrame {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int, false),
        Field::new("name", DataType::String, true),
        Field::new("is_important", DataType::Bool, true),
    ])
    .expect("Schema should be valid");

    let columns = if non_empty {
        vec![
            Column::int(vec![1, 2, 3, 4, 5]),
            Column::string(vec![
                String::from("Adam"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("Dan"),
                String::from("Evie"),
            ]),
            Column::bool(vec![false, false, true, false, true]),
        ]
    } else {
        vec![
            Column::int(vec![]),
            Column::string(vec![]),
            Column::bool(vec![]),
        ]
    };

    DataFrame::new(schema, columns).unwrap()
}

/// Returns the schema of sample dataframe
///
/// The schema is as follows:
///
/// 1. "id" - Int - non-null
/// 2. "name" - String - nullable
/// 3. "is_important" - Bool - nullable
///
/// See [`sample_dataframe`] for more information.
pub fn sample_dataframe_schema() -> Schema {
    Schema::new(vec![
        Field::new("id", DataType::Int, false),
        Field::new("name", DataType::String, true),
        Field::new("is_important", DataType::Bool, true),
    ])
    .expect("Schema should be valid")
}
