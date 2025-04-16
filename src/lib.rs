//! # JSON Diff View
//! 
//! A library for visually displaying differences between JSON structures in a human-readable format.
//! The tool recursively compares JSON structures and presents the result in a specially formatted 
//! text that makes all changes intuitively understandable.
//!
//! ## Features
//!
//! - Detection of added elements (marked with `[+]`)
//! - Detection of deleted elements (marked with `[-]`)
//! - Display of value changes in the format `"old" => "new"`
//! - Recursive comparison of nested objects and arrays
//! - Intelligent matching of similar elements using Levenshtein distance

mod core;

// Conditionally include Python bindings module
#[cfg(feature = "python-bindings")]
mod python;

// Re-export the main functions
pub use core::{compare_json, format_diff_to_string};

// Export the Python module if python-bindings feature is enabled
#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;

#[cfg(feature = "python-bindings")]
#[pymodule]
fn json_diff_view(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(python::compare_json_strings, py)?)?;
    m.add_function(wrap_pyfunction!(python::compare_json_files, py)?)?;
    m.add_function(wrap_pyfunction!(python::compare_json_values, py)?)?;
    Ok(())
}