//! Example of generating JSON Schema for CIK type

use cik::CIK;
use schemars::schema_for;

fn main() {
    // Generate the JSON schema for CIK
    let schema = schema_for!(CIK);

    // Convert to JSON and print
    let schema_json = serde_json::to_string_pretty(&schema).unwrap();
    println!("JSON Schema for CIK:");
    println!("{}", schema_json);
}
