//! Command-line interface for JSON Diff View

use json_diff_view::{compare_json, format_diff_to_string};
use std::fs;
use std::error::Error;
use clap::{Parser, ArgAction};
use serde_json::Value;

/// A tool for visually displaying differences between JSON files
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the original JSON file
    #[arg(index = 1)]
    before_file: String,

    /// Path to the modified JSON file
    #[arg(index = 2)]
    after_file: String,

    /// Output raw JSON diff without formatting
    #[arg(short, long, action = ArgAction::SetTrue)]
    raw: bool,

    /// Add auto-incremental index field to objects in arrays
    #[arg(long, action = ArgAction::SetTrue)]
    add_idx: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments using clap
    let cli = Cli::parse();
    
    // Read and parse JSON files
    let before_text = fs::read_to_string(&cli.before_file)
        .map_err(|e| format!("Failed to read file {}: {}", cli.before_file, e))?;
    
    let after_text = fs::read_to_string(&cli.after_file)
        .map_err(|e| format!("Failed to read file {}: {}", cli.after_file, e))?;
    
    let before: Value = serde_json::from_str(&before_text)
        .map_err(|e| format!("Failed to parse JSON from {}: {}", cli.before_file, e))?;
    
    let after: Value = serde_json::from_str(&after_text)
        .map_err(|e| format!("Failed to parse JSON from {}: {}", cli.after_file, e))?;
    
    // Compare JSON structures
    let result = compare_json(&before, &after, Some(cli.add_idx));
    
    // Format and output the result
    if cli.raw {
        // Output raw JSON
        let json_str = serde_json::to_string_pretty(&result)?;
        println!("{}", json_str);
    } else {
        // Format with special diff formatting
        let formatted_output = format_diff_to_string(&result, 0);
        println!("{}", formatted_output);
    }
    
    Ok(())
}
