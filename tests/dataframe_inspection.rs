use minidf::{Column, DataFrame, DataType, Field, Schema};

#[test]
fn dataframe_exposes_schema_metadata() {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int, false),
        Field::new("name", DataType::String, true),
    ])
    .expect("Schema should be valid");
    let columns = vec![
        Column::int(vec![1, 2]),
        Column::string_nullable(vec![Some("A".to_string()), None]),
    ];

    let df = DataFrame::new(schema, columns).expect("Dataframe should be valid");

    let df_schema_fields = df.schema().fields();

    assert_eq!(df.schema().len(), 2);

    assert!(df_schema_fields[0].name() == "id");
    assert!(df_schema_fields[0].dtype() == DataType::Int);
    assert!(!df_schema_fields[0].nullable());

    assert!(df_schema_fields[1].name() == "name");
    assert!(df_schema_fields[1].dtype() == DataType::String);
    assert!(df_schema_fields[1].nullable());
}

#[test]
fn dataframe_reports_shape() {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int, false),
        Field::new("name", DataType::String, true),
        Field::new("score", DataType::Float, true),
    ])
    .expect("Schema should be valid");
    let columns = vec![
        Column::int(vec![1, 2, 3, 4]),
        Column::string_nullable(vec![
            Some("A".to_string()),
            None,
            Some("C".to_string()),
            None,
        ]),
        Column::float(vec![9.9, 7.3, 2.2, 0.]),
    ];

    let df = DataFrame::new(schema, columns).expect("Dataframe should be valid");

    assert_eq!(df.width(), 3);
    assert_eq!(df.height(), 4);
}

#[test]
fn empty_dataframe_reports_zero_shape_and_empty_schema() {
    let schema = Schema::new(vec![]).expect("Empty schema is valid");
    let df = DataFrame::new(schema, vec![]).expect("Zero-width dataframe is valid");

    assert!(df.schema().is_empty());
    assert_eq!(df.width(), 0);
    assert_eq!(df.height(), 0);
}
