mod common;

use common::sample_dataframe;
use minidf::{Column, DataFrame, Schema};

use crate::common::sample_dataframe_schema;

#[test]
fn regular_dataframe_prints_correctly() {
    let df = sample_dataframe(true);
    let rendered_df = df.to_string();

    let target = String::from(
        "shape: (5, 3)\n\
        id:Int! | name:String? | is_important:Bool?\n\
        1 | Adam | false\n\
        2 | null | false\n\
        3 | Carol | true\n\
        4 | Dan | null\n\
        5 | Evie | true\n",
    );

    assert_eq!(rendered_df, target);
}

#[test]
fn zero_height_dataframe_prints_correctly() {
    let df = sample_dataframe(false);
    let rendered_df = df.to_string();

    let target = String::from(
        "shape: (0, 3)\n\
        id:Int! | name:String? | is_important:Bool?\n",
    );

    assert_eq!(rendered_df, target);
}

#[test]
fn blank_schema_dataframe_prints_correctly() {
    let df = DataFrame::new(Schema::new(vec![]).expect("Empty schema is valid"), vec![])
        .expect("Blank schema dataframe is valid");
    let rendered_df = df.to_string();

    let target = String::from("shape: (0, 0)\n");

    assert_eq!(rendered_df, target);
}

#[test]
fn long_string_values_are_truncated() {
    let schema = sample_dataframe_schema();
    let columns = vec![
        Column::int(vec![1, 2]),
        Column::string(vec![
            String::from("A very long name which certainly extends beyond 20 characters"),
            String::from("A shorter name"),
        ]),
        Column::bool(vec![true, false]),
    ];

    let df = DataFrame::new(schema, columns).expect("Dataframe is valid");

    let rendered_df = df.to_string();
    let target = String::from(
        "shape: (0, 3)\n\
        id:Int! | name:String? | is_important:Bool?\n\
        1 | A very long name ... | true\n\
        2 | A shorter name | false\n",
    );

    assert_eq!(rendered_df, target);
}
