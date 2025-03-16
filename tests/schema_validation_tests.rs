use std::path::Path;
use std::fs;

use aidir::validator;
use serde_json::Value;

// Test that validates the sample schema against the JSON schema
#[test]
fn test_validate_sample_schema_against_json_schema() {
    let schema_path = Path::new("examples/generic-saas/input/sample_schema.json");
    let json_schema_path = Path::new("schema.json");
    
    // Ensure both files exist
    assert!(schema_path.exists(), "Sample schema file not found");
    assert!(json_schema_path.exists(), "JSON schema file not found");
    
    // Read the sample schema
    let schema_content = fs::read_to_string(schema_path).expect("Failed to read sample schema");
    let schema_value: Value = serde_json::from_str(&schema_content).expect("Failed to parse sample schema");
    
    // Read the JSON schema
    let json_schema_content = fs::read_to_string(json_schema_path).expect("Failed to read JSON schema");
    let json_schema: Value = serde_json::from_str(&json_schema_content).expect("Failed to parse JSON schema");
    
    // Validate the sample schema against the JSON schema
    let result = jsonschema::validate(&json_schema, &schema_value);
    assert!(result.is_ok(), "Schema validation failed: {:?}", result.err());
}

// Test that validates the sample schema using our validator
#[test]
fn test_validate_sample_schema_with_validator() {
    let schema_path = Path::new("examples/generic-saas/input/sample_schema.json");
    
    // Ensure the schema file exists
    assert!(schema_path.exists(), "Sample schema file not found");
    
    // Validate the schema using our validator
    let result = validator::validate_schema_file(schema_path);
    assert!(result.is_ok(), "Schema validation failed: {:?}", result.err());
}

// Test that validates a modified schema with errors
#[test]
fn test_validate_invalid_schema() {
    // Create an invalid schema (missing required fields)
    let invalid_schema = serde_json::json!({
        "name": "Invalid Schema",
        "description": "Missing version field and empty directories",
        "directories": []
    });
    
    // Validate the schema using our validator
    let result = validator::validate_schema(&invalid_schema);
    assert!(result.is_err(), "Expected validation to fail for invalid schema");
    
    // Check the error type
    match result {
        Err(err) => {
            let err_string = format!("{:?}", err);
            assert!(err_string.contains("version") || err_string.contains("empty"),
                   "Error should mention missing version or empty directories");
        },
        _ => panic!("Expected error"),
    }
}

// Test that validates a schema with invalid directory structure
#[test]
fn test_validate_schema_with_invalid_directory() {
    // Create a schema with an invalid directory (missing description)
    let invalid_schema = serde_json::json!({
        "name": "Invalid Directory Schema",
        "description": "Schema with invalid directory",
        "version": "1.0.0",
        "directories": [
            {
                "name": "valid-dir",
                "description": "This directory is valid"
            },
            {
                "name": "invalid-dir"
                // Missing description field
            }
        ]
    });
    
    // Validate the schema using our validator
    let result = validator::validate_schema(&invalid_schema);
    assert!(result.is_err(), "Expected validation to fail for schema with invalid directory");
    
    // Check the error type
    match result {
        Err(err) => {
            let err_string = format!("{:?}", err);
            assert!(err_string.contains("description"),
                   "Error should mention missing description field");
        },
        _ => panic!("Expected error"),
    }
}

// Test that validates a schema with invalid subdirectory structure
#[test]
fn test_validate_schema_with_invalid_subdirectory() {
    // Create a schema with an invalid subdirectory (subdirectories is not an array)
    let invalid_schema = serde_json::json!({
        "name": "Invalid Subdirectory Schema",
        "description": "Schema with invalid subdirectory",
        "version": "1.0.0",
        "directories": [
            {
                "name": "parent-dir",
                "description": "Parent directory",
                "subdirectories": "not-an-array" // Should be an array
            }
        ]
    });
    
    // Validate the schema using our validator
    let result = validator::validate_schema(&invalid_schema);
    assert!(result.is_err(), "Expected validation to fail for schema with invalid subdirectory");
    
    // Check the error type
    match result {
        Err(err) => {
            let err_string = format!("{:?}", err);
            assert!(err_string.contains("subdirectories"),
                   "Error should mention invalid subdirectories field");
        },
        _ => panic!("Expected error"),
    }
} 