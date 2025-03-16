use crate::error::{AppError, Result};
use jsonschema;
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Validates a JSON schema file against the schema definition
pub fn validate_schema_file(schema_path: &Path) -> Result<()> {
    let schema_content = fs::read_to_string(schema_path)
        .map_err(|e| AppError::Io(e))?;
    
    let schema_value: Value = serde_json::from_str(&schema_content)
        .map_err(|e| AppError::Json(e))?;
    
    validate_schema(&schema_value)
}

/// Validates a JSON schema value against the schema definition
pub fn validate_schema(schema: &Value) -> Result<()> {
    // Use jsonschema's meta validation to validate the schema
    if !jsonschema::meta::is_valid(schema) {
        match jsonschema::meta::validate(schema) {
            Ok(_) => {}, // Should not happen since is_valid returned false
            Err(e) => return Err(AppError::SchemaValidation(e.to_string())),
        }
    }
    
    // Additional custom validation for our specific schema requirements
    validate_directory_structure(schema)?;
    
    Ok(())
}

/// Validates that the schema conforms to our directory structure requirements
fn validate_directory_structure(schema: &Value) -> Result<()> {
    // Check for required top-level fields
    let obj = schema.as_object().ok_or_else(|| 
        AppError::InvalidSchema("Schema must be a JSON object".to_string()))?;
    
    // Check for required fields
    for field in &["name", "description", "version", "directories"] {
        if !obj.contains_key(*field) {
            return Err(AppError::MissingField(format!("Missing required field: {}", field)));
        }
    }
    
    // Validate directories array
    if let Some(directories) = obj.get("directories") {
        if !directories.is_array() {
            return Err(AppError::InvalidSchema("'directories' must be an array".to_string()));
        }
        
        let directories_arr = directories.as_array().unwrap();
        if directories_arr.is_empty() {
            return Err(AppError::InvalidSchema("'directories' array cannot be empty".to_string()));
        }
        
        // Validate each directory
        for (i, dir) in directories_arr.iter().enumerate() {
            validate_directory(dir, i)?;
        }
    }
    
    Ok(())
}

/// Validates a single directory entry
fn validate_directory(dir: &Value, index: usize) -> Result<()> {
    let obj = dir.as_object().ok_or_else(|| 
        AppError::InvalidSchema(format!("Directory at index {} must be a JSON object", index)))?;
    
    // Check for required fields
    for field in &["name", "description"] {
        if !obj.contains_key(*field) {
            return Err(AppError::MissingField(
                format!("Directory at index {} is missing required field: {}", index, field)
            ));
        }
    }
    
    // Validate subdirectories if present
    if let Some(subdirs) = obj.get("subdirectories") {
        if !subdirs.is_array() {
            return Err(AppError::InvalidSchema(
                format!("'subdirectories' in directory at index {} must be an array", index)
            ));
        }
        
        let subdirs_arr = subdirs.as_array().unwrap();
        
        // Validate each subdirectory
        for (i, subdir) in subdirs_arr.iter().enumerate() {
            validate_directory(subdir, i)?;
        }
    }
    
    Ok(())
} 