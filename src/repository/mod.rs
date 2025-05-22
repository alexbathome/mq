use anyhow::Result;
use rayon::prelude::*;
use serde_json::Value;
use std::path::Path;
use walkdir::WalkDir;

use crate::filter::run_query;

/// Read and parse all YAML/JSON objects from a directory path
///
/// This function walks through a directory recursively, finds all YAML/JSON files,
/// reads them, parses them, and returns a vector of JSON values.
///
pub fn read_json_objects(repo_path: &Path) -> Result<Vec<Value>> {
    // Get all YAML/JSON files in the directory
    let walker = WalkDir::new(repo_path).into_iter();
    let files: Vec<_> = walker
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|ext| matches!(ext, "yaml" | "yml" | "json"))
                .unwrap_or(false)
        })
        .collect();

    // Process files in parallel using rayon
    let objects: Vec<Value> = files
        .par_iter()
        .flat_map(|entry| {
            let path = entry.path();
            let content = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => return Vec::new(), // Skip files we can't read
            };

            // Check file extension to determine parsing method
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

            if extension == "json" {
                // Parse JSON file
                match serde_json::from_str::<Value>(&content) {
                    Ok(value) => vec![value],
                    Err(_) => Vec::new(), // Skip invalid JSON
                }
            } else {
                // Parse YAML file, which could contain multiple documents
                process_yaml_content(&content)
            }
        })
        .collect();

    Ok(objects)
}

/// Read and parse all YAML/JSON objects from a directory path and apply a query
///
/// This function walks through a directory recursively, finds all YAML/JSON files,
/// reads them, parses them, applies a query, and returns a vector of JSON values.
///
pub fn read_json_objects_with_query(repo_path: &Path, query: &str) -> Result<Vec<Value>> {
    // Get all YAML/JSON files in the directory
    let walker = WalkDir::new(repo_path).into_iter();
    let files: Vec<_> = walker
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|ext| matches!(ext, "yaml" | "yml" | "json"))
                .unwrap_or(false)
        })
        .collect();

    // Process files in parallel using rayon
    let objects: Vec<Value> = files
        .par_iter()
        .flat_map(|entry| {
            let path = entry.path();
            let content = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => return Vec::new(), // Skip files we can't read
            };

            // Check file extension to determine parsing method
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

            let parsed_objects = if extension == "json" {
                // Parse JSON file
                match serde_json::from_str::<Value>(&content) {
                    Ok(value) => vec![value],
                    Err(_) => Vec::new(), // Skip invalid JSON
                }
            } else {
                // Parse YAML file, which could contain multiple documents
                process_yaml_content(&content)
            };

            // Apply the query to each parsed object
            parsed_objects
                .into_iter()
                .filter_map(|obj| apply_query(&obj, query))
                .collect::<Vec<_>>()
        })
        .collect();

    Ok(objects)
}

/// Process YAML content which might contain multiple documents
fn process_yaml_content(content: &str) -> Vec<Value> {
    let mut documents = Vec::new();

    // YAML can have multiple documents separated by ---
    for document in content.split("---") {
        if document.trim().is_empty() {
            continue; // Skip empty documents
        }

        if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(document) {
            if !yaml.is_null() {
                // Skip empty/null documents
                if let Ok(json) = serde_json::to_value(yaml) {
                    documents.push(json);
                }
            }
        }
    }

    documents
}

/// Apply a query to a JSON object
fn apply_query(obj: &Value, query: &str) -> Option<Value> {
    // Convert the input object to a jaq_json::Val
    let input = vec![obj.clone()];

    // Run the query using the run_query function
    match run_query(input, query) {
        Ok(results) => {
            // Convert the results using Val::into
            let converted_results: Vec<Value> = results.into_iter().map(|val| val.into()).collect();

            // Return the first result, if any
            converted_results.into_iter().next()
        }
        Err(_) => None, // Return None if the query fails
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::path::Path;

    #[test]
    fn test_read_json_objects_with_query() {
        let repo_path = Path::new("./testdata");
        let query = ".metadata.name";

        let result = read_json_objects_with_query(repo_path, query).unwrap();

        // Validate that the query extracts the correct names
        let expected_names = vec![
            json!("example-deployment"),
            json!("example-pv"),
            json!("example-service"),
            json!("example-secret"),
            json!("example-pvc"),
            json!("example-namespace"),
            json!("example-sa"),
            json!("example-config"),
            json!("example-job"),
            json!("example-ingress"),
            json!("example-cronjob"),
        ];

        assert_eq!(result, expected_names);
    }

    #[test]
    fn test_apply_query() {
        let input = json!({
            "kind": "Deployment",
            "metadata": { "name": "example-deployment" }
        });

        let query = ".metadata.name";
        let result = apply_query(&input, query);

        assert_eq!(result, Some(json!("example-deployment")));
    }
}
