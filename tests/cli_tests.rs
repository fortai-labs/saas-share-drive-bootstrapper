use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;
use std::path::Path;

// Helper function to create a sample schema file
fn create_sample_schema(dir: &Path) -> std::path::PathBuf {
    let schema_path = dir.join("schema.json");
    let schema_content = r#"{
        "name": "Test Schema",
        "description": "A test schema for integration tests",
        "version": "1.0.0",
        "directories": [
            {
                "name": "test-dir",
                "description": "A test directory"
            }
        ]
    }"#;
    
    fs::write(&schema_path, schema_content).unwrap();
    schema_path
}

#[test]
fn test_validate_valid_schema() {
    let temp_dir = TempDir::new().unwrap();
    let schema_path = create_sample_schema(temp_dir.path());
    
    let mut cmd = Command::cargo_bin("aidir").unwrap();
    
    cmd.arg("validate")
        .arg("--schema")
        .arg(schema_path);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Schema validation successful"));
}

#[test]
fn test_validate_invalid_schema() {
    let temp_dir = TempDir::new().unwrap();
    let schema_path = temp_dir.path().join("invalid_schema.json");
    
    // Create an invalid schema (missing required fields)
    let invalid_schema = r#"{
        "name": "Invalid Schema",
        "description": "Missing version field",
        "directories": []
    }"#;
    
    fs::write(&schema_path, invalid_schema).unwrap();
    
    let mut cmd = Command::cargo_bin("aidir").unwrap();
    
    cmd.arg("validate")
        .arg("--schema")
        .arg(schema_path);
    
    cmd.assert()
        .failure();
}

#[test]
fn test_create_directory_structure() {
    let temp_dir = TempDir::new().unwrap();
    let schema_path = create_sample_schema(temp_dir.path());
    let output_dir = temp_dir.path().join("output");
    
    let mut cmd = Command::cargo_bin("aidir").unwrap();
    
    cmd.arg("create")
        .arg("--schema")
        .arg(schema_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--organization")
        .arg("Test Organization");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Directory structure created successfully"));
    
    // Verify the output
    assert!(output_dir.exists());
    assert!(output_dir.join("README.md").exists());
    assert!(output_dir.join("test-dir").exists());
    assert!(output_dir.join("test-dir").join("README.md").exists());
}

#[test]
fn test_generate_sample_schema() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("generated_schema.json");
    
    let mut cmd = Command::cargo_bin("aidir").unwrap();
    
    cmd.arg("generate")
        .arg("--output")
        .arg(&output_path);
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Sample schema generated successfully"));
    
    // Verify the output
    assert!(output_path.exists());
    
    // Validate the generated schema
    let mut validate_cmd = Command::cargo_bin("aidir").unwrap();
    
    validate_cmd.arg("validate")
        .arg("--schema")
        .arg(&output_path);
    
    validate_cmd.assert()
        .success()
        .stdout(predicate::str::contains("Schema validation successful"));
}

#[test]
fn test_create_with_skip_validation() {
    let temp_dir = TempDir::new().unwrap();
    let schema_path = create_sample_schema(temp_dir.path());
    let output_dir = temp_dir.path().join("output");
    
    let mut cmd = Command::cargo_bin("aidir").unwrap();
    
    cmd.arg("create")
        .arg("--schema")
        .arg(schema_path)
        .arg("--output")
        .arg(&output_dir)
        .arg("--skip-validation");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Schema validation skipped"));
}

#[test]
fn test_nonexistent_schema_file() {
    let mut cmd = Command::cargo_bin("aidir").unwrap();
    
    cmd.arg("validate")
        .arg("--schema")
        .arg("nonexistent_file.json");
    
    cmd.assert()
        .failure();
} 