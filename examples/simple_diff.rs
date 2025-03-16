//! A simple example demonstrating how to use the JSON Diff View library

use json_diff_view::{compare_json, format_diff_to_string};
use serde_json::Value;

fn main() {
    // Example JSON structures
    let before_json = r#"
    {
        "name": "John Doe",
        "age": 30,
        "address": {
            "city": "New York",
            "zip": "10001"
        },
        "hobbies": ["reading", "gaming", "hiking"]
    }
    "#;

    let after_json = r#"
    {
        "name": "John Smith",
        "age": 31,
        "address": {
            "city": "Boston",
            "zip": "02101",
            "state": "MA"
        },
        "hobbies": ["reading", "swimming", "hiking"]
    }
    "#;

    // Parse JSON
    let before: Value = serde_json::from_str(before_json).unwrap();
    let after: Value = serde_json::from_str(after_json).unwrap();

    // Compare JSON structures
    let diff = compare_json(&before, &after);

    // Format the result
    let formatted = format_diff_to_string(&diff, 0);

    // Print the result
    println!("JSON Diff Result:");
    println!("{}", formatted);
}
