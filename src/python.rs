// Python bindings using PyO3
// Only compiled when the "python-bindings" feature is enabled

use std::fs;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::exceptions::PyValueError;
use serde_json::Value;

use crate::core::{compare_json, format_diff_to_string};

/// Compare two JSON strings and return a formatted string showing the differences
#[pyfunction]
#[pyo3(signature = (before_json, after_json, add_idx=None))]
pub fn compare_json_strings(before_json: &str, after_json: &str, add_idx: Option<bool>) -> PyResult<String> {
    // Parse JSON strings
    let before = serde_json::from_str(before_json)
        .map_err(|e| PyValueError::new_err(format!("Failed to parse 'before' JSON: {}", e)))?;
    
    let after = serde_json::from_str(after_json)
        .map_err(|e| PyValueError::new_err(format!("Failed to parse 'after' JSON: {}", e)))?;
    
    // Compare and format
    let result = compare_json(&before, &after, add_idx);
    let formatted = format_diff_to_string(&result, 0);
    
    Ok(formatted)
}

/// Compare two Python objects that can be converted to JSON and return a formatted string
/// showing the differences
#[pyfunction]
#[pyo3(signature = (before_obj, after_obj, add_idx=None))]
pub fn compare_json_values(py: Python, before_obj: PyObject, after_obj: PyObject, add_idx: Option<bool>) -> PyResult<String> {
    // Convert Python objects to JSON strings using Python's json module
    let json = PyModule::import(py, "json")?;
    
    let before_json = json.getattr("dumps")?.call1((before_obj,))?
        .extract::<String>()?;
    
    let after_json = json.getattr("dumps")?.call1((after_obj,))?
        .extract::<String>()?;
    
    // Use the string comparison function
    compare_json_strings(&before_json, &after_json, add_idx)
}

/// Compare two JSON files and return a formatted string showing the differences
#[pyfunction]
#[pyo3(signature = (before_path, after_path, add_idx=None))]
pub fn compare_json_files(before_path: &str, after_path: &str, add_idx: Option<bool>) -> PyResult<String> {
    // Read files
    let before_text = fs::read_to_string(before_path)
        .map_err(|e| PyValueError::new_err(format!("Failed to read file {}: {}", before_path, e)))?;
    
    let after_text = fs::read_to_string(after_path)
        .map_err(|e| PyValueError::new_err(format!("Failed to read file {}: {}", after_path, e)))?;
    
    // Parse JSON
    let before: Value = serde_json::from_str(&before_text)
        .map_err(|e| PyValueError::new_err(format!("Failed to parse JSON from {}: {}", before_path, e)))?;
    
    let after: Value = serde_json::from_str(&after_text)
        .map_err(|e| PyValueError::new_err(format!("Failed to parse JSON from {}: {}", after_path, e)))?;
    
    // Compare and format
    let result = compare_json(&before, &after, add_idx);
    let formatted = format_diff_to_string(&result, 0);
    
    Ok(formatted)
}
