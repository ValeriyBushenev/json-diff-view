//! Core functionality for JSON diffing and formatting

use serde_json::{Value, Map};
use std::cmp::{min, max};
use std::collections::HashSet;

/// Compare two JSON structures and return a Value representing the differences
///
/// # Arguments
/// * `before` - The original JSON structure
/// * `after` - The modified JSON structure
/// * `add_idx` - Optional parameter to add auto-incremental index field to objects in arrays
///
/// # Returns
/// A Value object representing the differences with:
/// * Added elements marked with `[+]`
/// * Deleted elements marked with `[-]`
/// * Changed values shown as `"old" => "new"`
pub fn compare_json(before: &Value, after: &Value, add_idx: Option<bool>) -> Value {
    let add_indexes = add_idx.unwrap_or(false);
    
    match (before, after) {
        // Compare objects
        (Value::Object(before_obj), Value::Object(after_obj)) => {
            let mut result = Map::new();
            
            // Check if this is an object with a changed identifier
            if let (Some((before_key, Value::String(before_id_str))), 
                    Some((after_key, Value::String(after_id_str)))) = 
                    (find_identifier_key_value(before_obj), find_identifier_key_value(after_obj)) {
                
                // If objects have the same identifier key but different values
                if before_key == after_key && before_id_str != after_id_str {
                    let similarity = string_similarity(&before_id_str, &after_id_str);
                    if similarity > 0.75 {
                        // This is the same object with a changed identifier
                        result.insert(before_key.clone(), 
                            Value::String(format!("{} => {}", before_id_str, after_id_str)));
                        
                        // Process the rest of the keys
                        let all_keys: HashSet<String> = before_obj.keys()
                            .chain(after_obj.keys())
                            .filter(|k| *k != &before_key)
                            .cloned()
                            .collect();
                        
                        for key in all_keys {
                            let before_val = before_obj.get(&key);
                            let after_val = after_obj.get(&key);
                            
                            match (before_val, after_val) {
                                (Some(b_val), Some(a_val)) => {
                                    if b_val == a_val {
                                        result.insert(key, b_val.clone());
                                    } else {
                                        result.insert(key, compare_json(b_val, a_val, Some(add_indexes)));
                                    }
                                },
                                (Some(b_val), None) => {
                                    result.insert(key, mark_deleted(b_val.clone()));
                                },
                                (None, Some(a_val)) => {
                                    result.insert(key, mark_added(a_val.clone()));
                                },
                                _ => unreachable!(),
                            }
                        }
                        
                        return Value::Object(result);
                    }
                }
            }
            
            // Standard object processing
            let all_keys: HashSet<String> = before_obj.keys()
                .chain(after_obj.keys())
                .cloned()
                .collect();
            
            for key in all_keys {
                let before_val = before_obj.get(&key);
                let after_val = after_obj.get(&key);
                
                match (before_val, after_val) {
                    (Some(b_val), Some(a_val)) => {
                        if b_val == a_val {
                            result.insert(key, b_val.clone());
                        } else {
                            result.insert(key, compare_json(b_val, a_val, Some(add_indexes)));
                        }
                    },
                    (Some(b_val), None) => {
                        result.insert(key, mark_deleted(b_val.clone()));
                    },
                    (None, Some(a_val)) => {
                        result.insert(key, mark_added(a_val.clone()));
                    },
                    _ => unreachable!(),
                }
            }
            
            Value::Object(result)
        },
        
        // Compare arrays
        (Value::Array(before_arr), Value::Array(after_arr)) => {
            let mut result = Vec::new();
            let mut matched_indices = vec![false; after_arr.len()];
            let similarity_threshold = 0.75;
            let mut object_index = 0;
            
            // Process elements from before_arr
            for before_item in before_arr {
                let mut best_match_idx = None;
                let mut best_similarity = 0.0;
                
                // If this is an object, find match by identifier
                if let Value::Object(before_obj) = before_item {
                    let before_id = get_identifier(before_obj);
                    
                    // Check each element from after_arr
                    for (i, after_item) in after_arr.iter().enumerate() {
                        if matched_indices[i] {
                            continue; // Element already matched
                        }
                        
                        if let Value::Object(after_obj) = after_item {
                            let after_id = get_identifier(after_obj);
                            
                            // If both elements have string identifiers
                            if let (Some(Value::String(before_str)), Some(Value::String(after_str))) = 
                                    (before_id.as_ref(), after_id) {
                                let similarity = string_similarity(before_str, &after_str);
                                if similarity > similarity_threshold && similarity > best_similarity {
                                    best_similarity = similarity;
                                    best_match_idx = Some(i);
                                }
                            }
                        }
                    }
                    
                    if let Some(idx) = best_match_idx {
                        // Found a match
                        matched_indices[idx] = true;
                        let mut compared = compare_json(before_item, &after_arr[idx], Some(add_indexes));
                        
                        // Add idx field if requested
                        if add_indexes {
                            if let Value::Object(obj) = &mut compared {
                                obj.insert("idx".to_string(), Value::Number(serde_json::Number::from(object_index)));
                            }
                        }
                        
                        result.push(compared);
                        object_index += 1;
                    } else {
                        // Element was deleted
                        let mut deleted = mark_deleted(before_item.clone());
                        
                        // Add idx field if requested
                        if add_indexes {
                            if let Value::Object(obj) = &mut deleted {
                                obj.insert("idx".to_string(), Value::Number(serde_json::Number::from(object_index)));
                            }
                        }
                        
                        result.push(deleted);
                        object_index += 1;
                    }
                } else {
                    // Not an object, look for exact match
                    let match_idx = after_arr.iter()
                        .position(|item| item == before_item);
                    
                    if let Some(idx) = match_idx {
                        matched_indices[idx] = true;
                        result.push(before_item.clone());
                    } else {
                        // Element was deleted
                        result.push(mark_deleted(before_item.clone()));
                    }
                }
            }
            
            // Add new elements from after_arr
            for (i, after_item) in after_arr.iter().enumerate() {
                if !matched_indices[i] {
                    // This is a new element
                    let mut added = mark_added(after_item.clone());
                    
                    // Add idx field if requested and it's an object
                    if add_indexes {
                        if let Value::Object(obj) = &mut added {
                            obj.insert("idx".to_string(), Value::Number(serde_json::Number::from(object_index)));
                            object_index += 1;
                        }
                    }
                    
                    result.push(added);
                }
            }
            
            Value::Array(result)
        },
        
        // Compare strings
        (Value::String(before_str), Value::String(after_str)) if before_str != after_str => {
            Value::String(format!("{} => {}", before_str, after_str))
        },
        
        // Compare other data types
        (before_val, after_val) if before_val != after_val => {
            Value::String(format!("{} => {}", before_val, after_val))
        },
        
        // Identical values
        (before_val, _) => {
            before_val.clone()
        },
    }
}

/// Format the comparison result into a human-readable string
///
/// # Arguments
/// * `value` - The Value object representing the differences
/// * `indent` - The base indentation level
///
/// # Returns
/// A formatted string with proper indentation and special markers for changes
pub fn format_diff_to_string(value: &Value, indent: usize) -> String {
    match value {
        Value::Object(obj) => {
            if obj.is_empty() {
                return "{}".to_string();
            }
            
            let mut result = String::from("{\n");
            let mut first = true;

            if let Some(idx_val) = obj.get("idx") {
                result.push_str(&" ".repeat(indent + 2));
                result.push_str(&format!("\"idx\":", ));
                
                let val_str = format_diff_to_string(idx_val, indent + 2);
                if val_str.contains('\n') {
                    result.push_str(&format!(" {}", val_str));
                } else {
                    result.push_str(&format!(" {}", val_str));
                }
                
                first = false;
            }
            
            for (key, val) in obj {
                if key == "idx" {
                    continue;
                }
                
                if !first {
                    result.push_str(",\n");
                }
                first = false;
                
                // Current level indentation
                result.push_str(&" ".repeat(indent + 2));
                
                // Key in quotes
                result.push_str(&format!("\"{}\":", key));
                
                // Value with increased indentation
                let val_str = format_diff_to_string(val, indent + 2);
                
                // If value is multiline, add space and value
                if val_str.contains('\n') {
                    result.push_str(&format!(" {}", val_str));
                } else {
                    result.push_str(&format!(" {}", val_str));
                }
            }
            
            // Closing brace with indentation
            result.push_str(&format!("\n{}}}", " ".repeat(indent)));
            result
        },
        Value::Array(arr) => {
            if arr.is_empty() {
                return "[]".to_string();
            }
            
            let mut result = String::from("[\n");
            let mut first = true;
            
            for item in arr {
                if !first {
                    result.push_str(",\n");
                }
                first = false;
                
                // Indentation for array elements
                result.push_str(&" ".repeat(indent + 2));
                
                // Add array element
                let item_str = format_diff_to_string(item, indent + 2);
                result.push_str(&item_str);
            }
            
            // Closing bracket
            result.push_str(&format!("\n{}]", " ".repeat(indent)));
            result
        },
        Value::String(s) => {
            // Check if string contains the " => " separator
            if let Some(idx) = s.find(" => ") {
                let before = &s[0..idx];
                let after = &s[(idx + 4)..];
                
                // Format as "before" => "after" without escaping quotes in JSON
                format!("\"{}\" => \"{}\"", before, after)
            } else if s.ends_with(" [+]") {
                // Added element
                let base = &s[0..(s.len() - 4)];
                format!("\"{}\" [+]", base)
            } else if s.ends_with(" [-]") {
                // Deleted element
                let base = &s[0..(s.len() - 4)];
                format!("\"{}\" [-]", base)
            } else {
                // Regular string
                format!("\"{}\"", s)
            }
        },
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
    }
}

// Calculate string similarity (0.0 - completely different, 1.0 - identical)
fn string_similarity(s1: &str, s2: &str) -> f64 {
    if s1 == s2 { return 1.0; }
    if s1.is_empty() || s2.is_empty() { return 0.0; }
    
    // Calculate Levenshtein distance
    let lev_dist = levenshtein_distance(s1, s2);
    let max_len = max(s1.len(), s2.len()) as f64;
    
    // Normalize: 1.0 - (distance / max_length)
    1.0 - (lev_dist as f64 / max_len)
}

// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    
    let m = s1_chars.len();
    let n = s2_chars.len();
    
    // Optimization: if strings are too different in length, return large distance
    if (m as isize - n as isize).abs() > (max(m, n) as f64 * 0.5) as isize {
        return max(m, n);
    }
    
    // Create distance matrix
    let mut dp = vec![vec![0; n+1]; m+1];
    
    // Initialize
    for i in 0..=m {
        dp[i][0] = i;
    }
    
    for j in 0..=n {
        dp[0][j] = j;
    }
    
    // Fill the matrix
    for i in 1..=m {
        for j in 1..=n {
            let cost = if s1_chars[i-1] == s2_chars[j-1] { 0 } else { 1 };
            dp[i][j] = min(
                min(dp[i-1][j] + 1, dp[i][j-1] + 1),
                dp[i-1][j-1] + cost
            );
        }
    }
    
    dp[m][n]
}

// Find key-value pair to use as object identifier
fn find_identifier_key_value(obj: &Map<String, Value>) -> Option<(String, Value)> {
    // First look for string fields
    for (key, value) in obj {
        if let Value::String(_) = value {
            return Some((key.clone(), value.clone()));
        }
    }
    
    // If no string fields, use number fields
    for (key, value) in obj {
        if let Value::Number(_) = value {
            return Some((key.clone(), value.clone()));
        }
    }
    
    // If no scalar values, take the first field
    obj.iter().next().map(|(key, value)| (key.clone(), value.clone()))
}

// Get object identifier value
fn get_identifier(obj: &Map<String, Value>) -> Option<Value> {
    find_identifier_key_value(obj).map(|(_, value)| value)
}

// Mark a deleted element
fn mark_deleted(mut value: Value) -> Value {
    if let Value::Object(obj) = &mut value {
        if let Some((key, Value::String(name))) = find_identifier_key_value(obj) {
            let marked_name = format!("{} [-]", name);
            obj.insert(key, Value::String(marked_name));
            return Value::Object(obj.clone());
        }
    } else if let Value::String(s) = &value {
        return Value::String(format!("{} [-]", s));
    }
    
    value
}

// Mark an added element
fn mark_added(mut value: Value) -> Value {
    if let Value::Object(obj) = &mut value {
        if let Some((key, Value::String(name))) = find_identifier_key_value(obj) {
            let marked_name = format!("{} [+]", name);
            obj.insert(key, Value::String(marked_name));
            return Value::Object(obj.clone());
        }
    } else if let Value::String(s) = &value {
        return Value::String(format!("{} [+]", s));
    }
    
    value
}
