#!/usr/bin/env python3
"""
Example showing how to use the json_diff_view Python module
"""

import json
import json_diff_view
import tempfile
import os

def example_with_strings():
    """Example comparing JSON strings"""
    print("\n=== Example with JSON strings ===")
    
    # Example JSON strings
    before_json = '{"name": "John Doe", "age": 30, "address": {"city": "New York", "zip": "10001"}}'
    after_json = '{"name": "John Smith", "age": 31, "address": {"city": "Boston", "zip": "02101", "state": "MA"}}'
    
    # Compare JSON strings
    result = json_diff_view.compare_json_strings(before_json, after_json)
    
    # Print the result
    print("Comparing JSON strings:")
    print(result)
    
    # Compare with array indexes included
    result_with_idx = json_diff_view.compare_json_strings(before_json, after_json, add_idx=True)
    
    # Print the result with indexes
    print("\nComparing JSON strings with array indexes:")
    print(result_with_idx)

def example_with_objects():
    """Example comparing Python objects"""
    print("\n=== Example with Python objects ===")
    
    # Example Python objects
    before_obj = {
        "name": "John Doe",
        "age": 30,
        "hobbies": ["reading", "gaming", "hiking"],
        "address": {
            "city": "New York",
            "zip": "10001"
        }
    }
    
    after_obj = {
        "name": "John Smith",
        "age": 31,
        "hobbies": ["reading", "swimming", "hiking"],
        "address": {
            "city": "Boston",
            "zip": "02101",
            "state": "MA"
        }
    }
    
    # Compare Python objects
    result = json_diff_view.compare_json_values(before_obj, after_obj)
    
    # Print the result
    print("Comparing Python objects:")
    print(result)
    
    # Compare with array indexes included
    result_with_idx = json_diff_view.compare_json_values(before_obj, after_obj, add_idx=True)
    
    # Print the result with indexes
    print("\nComparing Python objects with array indexes:")
    print(result_with_idx)

def example_with_files():
    """Example comparing JSON files"""
    print("\n=== Example with JSON files ===")
    
    # Create temporary files
    with tempfile.TemporaryDirectory() as tmp_dir:
        # Create JSON files
        before_file = os.path.join(tmp_dir, "before.json")
        after_file = os.path.join(tmp_dir, "after.json")
        
        before_content = {
            "users": [
                {"id": 1, "name": "Alice"},
                {"id": 2, "name": "Bob"}
            ]
        }
        
        after_content = {
            "users": [
                {"id": 1, "name": "Alice", "role": "admin"},
                {"id": 3, "name": "Charlie"}
            ]
        }
        
        # Write content to files
        with open(before_file, "w") as f:
            json.dump(before_content, f, indent=2)
        
        with open(after_file, "w") as f:
            json.dump(after_content, f, indent=2)
        
        # Compare files
        result = json_diff_view.compare_json_files(before_file, after_file)
        
        # Print the result
        print(f"Comparing files {before_file} and {after_file}:")
        print(result)
        
        # Compare files with array indexes included
        result_with_idx = json_diff_view.compare_json_files(before_file, after_file, add_idx=True)
        
        # Print the result with indexes
        print(f"\nComparing files with array indexes:")
        print(result_with_idx)

def main():
    """Run all examples"""
    print("JSON Diff View Python Examples")
    print("==============================")
    
    example_with_strings()
    example_with_objects()
    example_with_files()

if __name__ == "__main__":
    main()
