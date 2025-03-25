# JSON Diff View - Python Bindings

Python bindings for the JSON Diff View library - a tool for visually displaying differences between JSON structures in a human-readable format.

## Installation

```bash
pip install json-diff-view
```

Or install from source:

```bash
# From the python directory
pip install .

# Or from the project root
pip install ./python
```

## Usage

```python
import json_diff_view

# Compare JSON strings
before_json = '{"name": "John", "age": 30}'
after_json = '{"name": "John", "age": 31}'
result = json_diff_view.compare_json_strings(before_json, after_json)
print(result)

# Compare with array indexes included in the output
result_with_idx = json_diff_view.compare_json_strings(before_json, after_json, add_idx=True)
print(result_with_idx)

# Compare Python objects directly
before_obj = {"name": "John", "age": 30}
after_obj = {"name": "John", "age": 31}
result = json_diff_view.compare_json_values(before_obj, after_obj)
print(result)

# Compare with array indexes included
result_with_idx = json_diff_view.compare_json_values(before_obj, after_obj, add_idx=True)
print(result_with_idx)

# Compare JSON files
result = json_diff_view.compare_json_files("before.json", "after.json")
print(result)

# Compare JSON files with array indexes included
result_with_idx = json_diff_view.compare_json_files("before.json", "after.json", add_idx=True)
print(result_with_idx)
```

## API Reference

### compare_json_strings(before_json, after_json, add_idx=None)

Compare two JSON strings and return a formatted string showing the differences.

- `before_json`: JSON string representing the "before" state
- `after_json`: JSON string representing the "after" state
- `add_idx`: Optional boolean to include array indexes in the output (default: None)

### compare_json_values(before_obj, after_obj, add_idx=None)

Compare two Python objects and return a formatted string showing the differences.

- `before_obj`: Python object representing the "before" state
- `after_obj`: Python object representing the "after" state
- `add_idx`: Optional boolean to include array indexes in the output (default: None)

### compare_json_files(before_path, after_path, add_idx=None)

Compare two JSON files and return a formatted string showing the differences.

- `before_path`: Path to the file representing the "before" state
- `after_path`: Path to the file representing the "after" state
- `add_idx`: Optional boolean to include array indexes in the output (default: None)

## Output Format

The result is formatted as a text showing differences:

```
{
  "name": "John",
  "age": 30 => 31
}
```

Where:
- `"old" => "new"` shows changed values
- `"value" [+]` shows added items
- `"value" [-]` shows removed items

When `add_idx=True` is used, array items will include their indexes:

```
{
  "items": [
    [0] "item1",
    [1] "item2" => "modified item"
  ]
}
```

## Platform-specific Wheels

If you prefer to use pre-compiled wheels instead of building from source:

```bash
# Install the most appropriate wheel for your system
python install.py

# Or force installation from source
python install.py --source

# Get verbose output
python install.py --verbose
```

## Building Wheels

To build platform-specific wheels:

```bash
python build-wheel.py
```

This will create a wheel in the `wheels/` directory with platform metadata.

## Development

This package is built using [PyO3](https://github.com/PyO3/pyo3) to bind the Rust implementation to Python. The Rust source code is located in the parent directory.