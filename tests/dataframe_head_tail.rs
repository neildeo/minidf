use minidf::{Column, DataFrame, DataType, Field, Schema};

#[test]
fn head_and_tail_on_regular_dataframe() {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int, false),
        Field::new("name", DataType::String, true),
        Field::new("is_important", DataType::Bool, true),
    ])
    .expect("Schema should be valid");

    let columns = vec![
        Column::int(vec![1, 2, 3, 4, 5]),
        Column::string(vec![
            String::from("Adam"),
            String::from("Bob"),
            String::from("Carol"),
            String::from("Dan"),
            String::from("Evie"),
        ]),
        Column::bool(vec![false, false, true, false, true]),
    ];

    let head_target_columns = vec![
        Column::int(vec![1, 2]),
        Column::string(vec![String::from("Adam"), String::from("Bob")]),
        Column::bool(vec![false, false]),
    ];

    let tail_target_columns = vec![
        Column::int(vec![4, 5]),
        Column::string(vec![String::from("Dan"), String::from("Evie")]),
        Column::bool(vec![false, true]),
    ];

    let main_df = DataFrame::new(schema.clone(), columns).expect("Dataframe should be valid");

    let head_target_df =
        DataFrame::new(schema.clone(), head_target_columns).expect("Dataframe should be valid");

    let tail_target_df =
        DataFrame::new(schema.clone(), tail_target_columns).expect("Dataframe should be valid");

    assert_eq!(main_df.head(2), head_target_df);
    assert_eq!(main_df.tail(2), tail_target_df);
}

#[test]
fn head_and_tail_0_returns_zero_height_df() {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int, false),
        Field::new("name", DataType::String, true),
        Field::new("is_important", DataType::Bool, true),
    ])
    .expect("Schema should be valid");

    let columns = vec![
        Column::int(vec![1, 2, 3, 4, 5]),
        Column::string(vec![
            String::from("Adam"),
            String::from("Bob"),
            String::from("Carol"),
            String::from("Dan"),
            String::from("Evie"),
        ]),
        Column::bool(vec![false, false, true, false, true]),
    ];

    let head_target_columns = vec![
        Column::int(vec![]),
        Column::string(vec![]),
        Column::bool(vec![]),
    ];

    let tail_target_columns = vec![
        Column::int(vec![]),
        Column::string(vec![]),
        Column::bool(vec![]),
    ];

    let main_df = DataFrame::new(schema.clone(), columns).expect("Dataframe should be valid");

    let head_target_df =
        DataFrame::new(schema.clone(), head_target_columns).expect("Dataframe should be valid");

    let tail_target_df =
        DataFrame::new(schema.clone(), tail_target_columns).expect("Dataframe should be valid");

    assert_eq!(main_df.head(0), head_target_df);
    assert_eq!(main_df.tail(0), tail_target_df);
}

#[test]
fn head_and_tail_n_exceeds_height() {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int, false),
        Field::new("name", DataType::String, true),
        Field::new("is_important", DataType::Bool, true),
    ])
    .expect("Schema should be valid");

    let columns = vec![
        Column::int(vec![1, 2, 3, 4, 5]),
        Column::string(vec![
            String::from("Adam"),
            String::from("Bob"),
            String::from("Carol"),
            String::from("Dan"),
            String::from("Evie"),
        ]),
        Column::bool(vec![false, false, true, false, true]),
    ];

    let main_df = DataFrame::new(schema.clone(), columns).expect("Dataframe should be valid");

    assert_eq!(main_df.head(5), main_df);
    assert_eq!(main_df.tail(5), main_df);
    assert_eq!(main_df.head(6), main_df);
    assert_eq!(main_df.tail(6), main_df);
}

#[test]
fn head_and_tail_on_zero_height_dataframe() {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int, false),
        Field::new("name", DataType::String, true),
        Field::new("is_important", DataType::Bool, true),
    ])
    .expect("Schema should be valid");

    let columns = vec![
        Column::int(vec![]),
        Column::string(vec![]),
        Column::bool(vec![]),
    ];

    let main_df = DataFrame::new(schema.clone(), columns).expect("Dataframe should be valid");

    assert_eq!(main_df.head(1).height(), 0);
    assert_eq!(main_df.head(1).schema(), &schema);
    assert_eq!(main_df.tail(1).height(), 0);
    assert_eq!(main_df.tail(1).schema(), &schema);
}

#[test]
fn head_and_tail_on_empty_schema_dataframe() {
    let blank_schema = Schema::new(vec![]).expect("Empty schema is valid");

    let empty_df = DataFrame::new(blank_schema, vec![]).expect("Blank-schema dataframe is valid");

    assert_eq!(empty_df.head(1), empty_df);
    assert_eq!(empty_df.tail(1), empty_df);
}
