mod common;

use common::{sample_dataframe, sample_dataframe_schema};

use minidf::{Column, DataFrame, Schema};

#[test]
fn head_and_tail_on_regular_dataframe() {
    let head_target_columns = vec![
        Column::int(vec![1, 2]),
        Column::string_nullable(vec![Some(String::from("Adam")), None]),
        Column::bool(vec![false, false]),
    ];

    let tail_target_columns = vec![
        Column::int(vec![4, 5]),
        Column::string(vec![String::from("Dan"), String::from("Evie")]),
        Column::bool_nullable(vec![None, Some(true)]),
    ];

    let main_df = sample_dataframe(true);

    let head_target_df = DataFrame::new(sample_dataframe_schema(), head_target_columns)
        .expect("Dataframe should be valid");

    let tail_target_df = DataFrame::new(sample_dataframe_schema(), tail_target_columns)
        .expect("Dataframe should be valid");

    assert_eq!(main_df.head(2), head_target_df);
    assert_eq!(main_df.tail(2), tail_target_df);
}

#[test]
fn head_and_tail_0_returns_zero_height_df() {
    let main_df = sample_dataframe(true);

    let head_target_df = sample_dataframe(false);

    let tail_target_df = sample_dataframe(false);

    assert_eq!(main_df.head(0), head_target_df);
    assert_eq!(main_df.tail(0), tail_target_df);
}

#[test]
fn head_and_tail_n_matches_or_exceeds_height() {
    let main_df = sample_dataframe(true);

    assert_eq!(main_df.head(5), main_df);
    assert_eq!(main_df.tail(5), main_df);
    assert_eq!(main_df.head(6), main_df);
    assert_eq!(main_df.tail(6), main_df);
}

#[test]
fn head_and_tail_on_zero_height_dataframe() {
    let main_df = sample_dataframe(false);
    let target_df = sample_dataframe(false);

    assert_eq!(main_df.head(1), target_df);
    assert_eq!(main_df.tail(1), target_df);
}

#[test]
fn head_and_tail_on_empty_schema_dataframe() {
    let blank_schema = Schema::new(vec![]).expect("Empty schema is valid");

    let empty_df = DataFrame::new(blank_schema, vec![]).expect("Blank-schema dataframe is valid");

    assert_eq!(empty_df.head(1), empty_df);
    assert_eq!(empty_df.tail(1), empty_df);
}
