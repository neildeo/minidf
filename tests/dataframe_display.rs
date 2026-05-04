mod common;

use common::sample_dataframe;
use minidf::{DataFrame, Schema};

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
