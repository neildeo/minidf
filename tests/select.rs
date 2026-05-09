mod common;

use common::sample_dataframe;
use minidf::{Column, DataFrame, DataType, Field, MiniDfError, Schema};

#[test]
fn select_one_column() {
    let df = sample_dataframe(true);

    let select_df = df.select(["id"]).expect("Selected column should be valid.");

    let target_schema =
        Schema::new(vec![Field::new("id", DataType::Int, false)]).expect("Target schema is valid");

    let target_df = DataFrame::new(target_schema, vec![Column::int(vec![1, 2, 3, 4, 5])])
        .expect("Target df is valid.");

    assert_eq!(select_df, target_df);
}

#[test]
fn select_multiple_columns_in_original_order() {
    let df = sample_dataframe(true);

    let select_df = df
        .select(["id", "name"])
        .expect("Selected columns should be valid.");

    let target_schema = Schema::new(vec![
        Field::new("id", DataType::Int, false),
        Field::new("name", DataType::String, true),
    ])
    .expect("Target schema is valid");

    let target_df = DataFrame::new(
        target_schema,
        vec![
            Column::int(vec![1, 2, 3, 4, 5]),
            Column::string_nullable(vec![
                Some(String::from("Adam")),
                None,
                Some(String::from("Carol")),
                Some(String::from("Dan")),
                Some(String::from("Evie")),
            ]),
        ],
    )
    .expect("Target df is valid.");

    assert_eq!(select_df, target_df);
}

#[test]
fn select_multiple_columns_in_different_order() {
    let df = sample_dataframe(true);

    let select_df = df
        .select(["name", "id"])
        .expect("Selected columns should be valid.");

    let target_schema = Schema::new(vec![
        Field::new("name", DataType::String, true),
        Field::new("id", DataType::Int, false),
    ])
    .expect("Target schema is valid");

    let target_df = DataFrame::new(
        target_schema,
        vec![
            Column::string_nullable(vec![
                Some(String::from("Adam")),
                None,
                Some(String::from("Carol")),
                Some(String::from("Dan")),
                Some(String::from("Evie")),
            ]),
            Column::int(vec![1, 2, 3, 4, 5]),
        ],
    )
    .expect("Target df is valid.");

    assert_eq!(select_df, target_df);
}

#[test]
fn select_from_zero_row_dataframe_works() {
    let df = sample_dataframe(false);

    let select_df = df
        .select(["id", "name"])
        .expect("Selected columns should be valid.");

    let target_schema = Schema::new(vec![
        Field::new("id", DataType::Int, false),
        Field::new("name", DataType::String, true),
    ])
    .expect("Target schema is valid");

    let target_df = DataFrame::new(
        target_schema,
        vec![Column::int(vec![]), Column::string_nullable(vec![])],
    )
    .expect("Target df is valid.");

    assert_eq!(select_df, target_df);
}

#[test]
fn select_errors_on_missing_column() {
    let df = sample_dataframe(true);

    let select_df = df.select(["id", "favourite_food"]);

    assert!(select_df.is_err_and(|e| {
        matches!(
            e, 
            MiniDfError::ColumnNotFound { name } if name == "favourite_food")
    }))
}

#[test]
fn select_errors_on_duplicate_column() {
    let df = sample_dataframe(true);

    let select_df = df.select(["id", "id"]);

    assert!(select_df.is_err_and(|e| {
        matches!(
            e, 
            MiniDfError::DuplicateColumnName { duplicate_name } if duplicate_name == "id")
    }))
}

#[test]
fn select_errors_on_zero_columns() {
    let df = sample_dataframe(true);

    let select_df = df.select(Vec::<String>::new());

    assert!(select_df.is_err_and(|e| { matches!(e, MiniDfError::EmptyColumnSelection) }))
}
