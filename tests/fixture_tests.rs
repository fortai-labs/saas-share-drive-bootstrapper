use std::path::Path;
use std::fs;
use std::path::PathBuf;

use aidir::models::DirectoryStructure;
use aidir::validator;

// Test that validates the sample schema from the examples directory
#[test]
fn test_validate_sample_schema() {
    let schema_path = Path::new("examples/generic-saas/input/sample_schema.json");
    
    // Ensure the schema file exists
    assert!(schema_path.exists(), "Sample schema file not found");
    
    // Validate the schema
    let result = validator::validate_schema_file(schema_path);
    assert!(result.is_ok(), "Sample schema validation failed: {:?}", result.err());
}

// Test that parses the sample schema and verifies its structure
#[test]
fn test_parse_sample_schema() {
    let schema_path = Path::new("examples/generic-saas/input/sample_schema.json");
    
    // Read and parse the schema
    let schema_content = fs::read_to_string(schema_path).expect("Failed to read sample schema");
    let structure: DirectoryStructure = serde_json::from_str(&schema_content).expect("Failed to parse sample schema");
    
    // Verify basic structure properties
    assert_eq!(structure.name, "SaaS_Company_Drive");
    assert!(structure.description.contains("Centralized repository"), "Description should mention 'Centralized repository'");
    assert!(!structure.directories.is_empty(), "Sample schema should contain directories");
    
    // Verify that all directories have names and descriptions
    for dir in &structure.directories {
        assert!(!dir.name.is_empty(), "Directory name should not be empty");
        assert!(!dir.description.is_empty(), "Directory description should not be empty");
    }
}

// Test that verifies the output directory structure matches what we expect
#[test]
fn test_verify_output_structure() {
    let output_dir = Path::new("examples/generic-saas/output");
    
    // Ensure the output directory exists
    assert!(output_dir.exists(), "Output directory not found");
    
    // Check for README.md in the root
    let readme_path = output_dir.join("README.md");
    assert!(readme_path.exists(), "Root README.md not found");
    
    // Read the README content
    let readme_content = fs::read_to_string(readme_path).expect("Failed to read README.md");
    assert!(readme_content.contains("SaaS_Company_Drive"), "README should contain the structure name");
    
    // Check for expected directories
    let expected_dirs = [
        "01_Administrative",
        "02_Product",
        "03_Engineering",
        "04_Sales_and_Marketing",
        "05_Data_and_Analytics",
        "06_Operations",
        "07_Internal_Communications",
        "08_Projects",
        "09_Archives"
    ];
    
    for dir_name in &expected_dirs {
        let dir_path = output_dir.join(dir_name);
        assert!(dir_path.exists() && dir_path.is_dir(), "Expected directory {} not found", dir_name);
        
        // Check for README.md in each directory
        let dir_readme = dir_path.join("README.md");
        assert!(dir_readme.exists(), "README.md not found in {}", dir_name);
    }
}

// Test that compares the input schema with the generated output
#[test]
fn test_schema_output_consistency() {
    let schema_path = Path::new("examples/generic-saas/input/sample_schema.json");
    let output_dir = Path::new("examples/generic-saas/output");
    
    // Read and parse the schema
    let schema_content = fs::read_to_string(schema_path).expect("Failed to read sample schema");
    let structure: DirectoryStructure = serde_json::from_str(&schema_content).expect("Failed to parse sample schema");
    
    // Verify that each directory in the schema has a corresponding directory in the output
    for dir in &structure.directories {
        // Extract the directory name, which might have a numeric prefix in the output
        let dir_name = &dir.name;
        
        // Find the corresponding directory in the output
        let mut found = false;
        for entry in fs::read_dir(output_dir).expect("Failed to read output directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            
            if path.is_dir() && path.file_name().unwrap().to_string_lossy().contains(dir_name) {
                found = true;
                
                // Check for README.md in the directory
                let readme_path = path.join("README.md");
                assert!(readme_path.exists(), "README.md not found in {}", dir_name);
                
                // Read the README content
                let readme_content = fs::read_to_string(readme_path).expect("Failed to read README.md");
                
                // Verify that the README contains the directory description
                assert!(readme_content.contains(&dir.description), 
                       "README for {} should contain its description", dir_name);
                
                break;
            }
        }
        
        assert!(found, "Directory {} from schema not found in output", dir_name);
    }
}

// Test that verifies subdirectories are correctly generated
#[test]
fn test_subdirectory_structure() {
    let output_dir = PathBuf::from("examples/generic-saas/output");
    
    // Check that specific subdirectories exist and have README.md files
    let subdirectories = vec![
        "01_Administrative/HR_and_People",
        "01_Administrative/Finance_and_Accounting",
        "01_Administrative/Legal_Documents",
        "02_Product/Product_Design",
        "02_Product/Product_Requirements",
        "03_Engineering/API_Documentation",
        "03_Engineering/Architecture",
        "03_Engineering/Database",
        "04_Sales_and_Marketing/Marketing/Brand_Assets",
        "04_Sales_and_Marketing/Marketing/Campaigns"
    ];
    
    for subdir in subdirectories {
        let subdir_path = output_dir.join(subdir);
        assert!(
            subdir_path.exists(),
            "Expected subdirectory {} not found",
            subdir
        );
        
        let readme_path = subdir_path.join("README.md");
        assert!(
            readme_path.exists(),
            "README.md not found in subdirectory {}",
            subdir
        );
    }
} 