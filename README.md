# JSON Diff View

A library and command-line tool for visually displaying differences between JSON structures in a human-readable format. The tool recursively compares JSON structures and presents the result in a specially formatted text that makes all changes intuitively understandable.

## Features

- Detection of added elements (marked with `[+]`)
- Detection of deleted elements (marked with `[-]`)
- Display of value changes in the format `"old" => "new"`
- Recursive comparison of nested objects and arrays
- Intelligent matching of similar elements using Levenshtein distance
- Abstract approach that works with any JSON structure without binding to specific fields
- Formatted output with proper indentation

## Installation

### Rust Library

```bash
cargo add json-diff-view
```

### Python Package

```bash
pip install json-diff-view
```

### Command-line Tool

```bash
cargo install json-diff-view
```

## Usage

### Rust Library

```rust
use json_diff_view::{compare_json, format_diff_to_string};
use serde_json::Value;

fn main() {
    // Parse your JSON strings or load from files
    let before: Value = serde_json::from_str(r#"{"name": "John"}"#).unwrap();
    let after: Value = serde_json::from_str(r#"{"name": "Jane"}"#).unwrap();
    
    // Compare
    let result = compare_json(&before, &after);
    
    // Format the result
    let formatted = format_diff_to_string(&result, 0);
    
    println!("{}", formatted);
}
```

### Python Package

```python
import json_diff_view

# Compare JSON strings
before = '{"name": "John"}'
after = '{"name": "Jane"}'
result = json_diff_view.compare_json_strings(before, after)
print(result)

# Or compare files
result = json_diff_view.compare_json_files("before.json", "after.json")
print(result)
```

### Command-line Tool

```bash
json-diff-view before.json after.json
```

## Output Format

The result is output in a special format:

```
{
  "key": {
    "changed_field": "old value" => "new value",
    "deleted_field": "value" [-],
    "added_field": "value" [+],
    "unchanged_field": "value"
  }
}
```

## Examples

### Example 1: Changing Values

**before.json**
```json
{
  "name": "John Doe",
  "age": 30,
  "address": {
    "city": "New York",
    "zip": "10001"
  }
}
```

**after.json**
```json
{
  "name": "John Smith",
  "age": 31,
  "address": {
    "city": "Boston",
    "zip": "02101"
  }
}
```

**Result**
```
{
  "name": "John Doe" => "John Smith",
  "age": 30 => 31,
  "address": {
    "city": "New York" => "Boston",
    "zip": "10001" => "02101"
  }
}
```

### Example 2: Adding and Removing Elements

**before.json**
```json
{
  "users": [
    {
      "id": 1,
      "name": "Alice"
    },
    {
      "id": 2,
      "name": "Bob"
    }
  ]
}
```

**after.json**
```json
{
  "users": [
    {
      "id": 1,
      "name": "Alice",
      "role": "admin"
    },
    {
      "id": 3,
      "name": "Charlie"
    }
  ]
}
```

**Result**
```
{
  "users": [
    {
      "id": 1,
      "name": "Alice",
      "role": "admin" [+]
    },
    {
      "id": 2,
      "name": "Bob" [-]
    },
    {
      "id": 3,
      "name": "Charlie" [+]
    }
  ]
}
```

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/your-username/json-diff-view.git
cd json-diff-view

# Build the Rust library
cargo build --release

# Build the Python extension (optional)
cd python
pip install -e .
```

## License

MIT