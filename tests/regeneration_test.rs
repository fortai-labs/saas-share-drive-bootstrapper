use std::path::{Path, PathBuf};
use std::fs;
use tempfile::TempDir;
use walkdir::WalkDir;

use aidir::models::DirectoryStructure;
use aidir::generator;

// Helper function to compare directory structures
fn compare_directories(dir1: &Path, dir2: &Path) -> Result<(), String> {
    // Get all files in both directories recursively
    let files1: Vec<PathBuf> = WalkDir::new(dir1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().strip_prefix(dir1).unwrap().to_path_buf())
        .collect();
    
    let files2: Vec<PathBuf> = WalkDir::new(dir2)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().strip_prefix(dir2).unwrap().to_path_buf())
        .collect();
    
    // Check if both directories have the same files
    for file in &files1 {
        if !files2.contains(file) {
            return Err(format!("File {:?} exists in original but not in regenerated", file));
        }
    }
    
    for file in &files2 {
        if !files1.contains(file) {
            return Err(format!("File {:?} exists in regenerated but not in original", file));
        }
    }
    
    // Compare file contents (only for README.md files to avoid timestamp differences)
    for file in &files1 {
        if file.extension().map_or(false, |ext| ext == "md") {
            let content1 = fs::read_to_string(dir1.join(file))
                .map_err(|e| format!("Failed to read {:?}: {}", file, e))?;
            
            let content2 = fs::read_to_string(dir2.join(file))
                .map_err(|e| format!("Failed to read {:?}: {}", file, e))?;
            
            // Compare only the essential parts of the content (excluding timestamps)
            let essential_content1 = extract_essential_content(&content1);
            let essential_content2 = extract_essential_content(&content2);
            
            if essential_content1 != essential_content2 {
                return Err(format!("Content differs for file {:?}", file));
            }
        }
    }
    
    Ok(())
}

// Helper function to extract essential content from README.md files
// This ignores timestamps and other variable data
fn extract_essential_content(content: &str) -> String {
    // Split the content by lines and filter out lines with timestamps or other variable data
    content.lines()
        .filter(|line| {
            !line.contains("generated_date") && 
            !line.contains("last_updated") &&
            !line.trim().starts_with("---") // Skip frontmatter delimiters
        })
        .collect::<Vec<&str>>()
        .join("\n")
}

#[test]
fn test_regenerate_output() {
    let schema_path = Path::new("examples/generic-saas/input/sample_schema.json");
    let original_output_dir = Path::new("examples/generic-saas/output");
    
    // Create a temporary directory for the regenerated output
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");
    let regenerated_output_dir = temp_dir.path();
    
    // Read and parse the schema
    let schema_content = fs::read_to_string(schema_path).expect("Failed to read sample schema");
    let structure: DirectoryStructure = serde_json::from_str(&schema_content).expect("Failed to parse sample schema");
    
    // Regenerate the output
    let result = generator::generate_structure(&structure, regenerated_output_dir, Some("SaaS Company"));
    assert!(result.is_ok(), "Failed to regenerate output: {:?}", result.err());
    
    // Compare the original and regenerated outputs
    let comparison_result = compare_directories(original_output_dir, regenerated_output_dir);
    assert!(comparison_result.is_ok(), "Directory comparison failed: {}", comparison_result.err().unwrap());
}

// Test that regenerates a specific subdirectory and verifies its structure
#[test]
fn test_regenerate_subdirectory() {
    let schema_path = Path::new("examples/generic-saas/input/sample_schema.json");
    
    // Create a temporary directory for the regenerated output
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");
    
    // Read and parse the schema
    let schema_content = fs::read_to_string(schema_path).expect("Failed to read sample schema");
    let structure: DirectoryStructure = serde_json::from_str(&schema_content).expect("Failed to parse sample schema");
    
    // Find the Engineering directory in the schema
    let engineering_dir = structure.directories.iter()
        .find(|dir| dir.name == "03_Engineering")
        .expect("Engineering directory not found in schema");
    
    // Regenerate just the Engineering directory
    let result = generator::generate_directory(
        &structure,
        engineering_dir,
        temp_dir.path(),
        Some("SaaS Company"),
        ""
    );
    assert!(result.is_ok(), "Failed to regenerate Engineering directory: {:?}", result.err());
    
    // Verify the regenerated directory structure
    let eng_dir_path = temp_dir.path().join("03_Engineering");
    assert!(eng_dir_path.exists() && eng_dir_path.is_dir(), "Engineering directory not created");
    
    // Check for README.md
    let readme_path = eng_dir_path.join("README.md");
    assert!(readme_path.exists(), "README.md not found in Engineering directory");
    
    // Check for expected subdirectories
    let expected_subdirs = ["API_Documentation", "Architecture", "Database", "DevOps", "Infrastructure"];
    
    for subdir in &expected_subdirs {
        let subdir_path = eng_dir_path.join(subdir);
        assert!(subdir_path.exists() && subdir_path.is_dir(), 
               "Expected subdirectory {} not found", subdir);
        
        // Check for README.md in each subdirectory
        let subdir_readme = subdir_path.join("README.md");
        assert!(subdir_readme.exists(), "README.md not found in {}", subdir);
    }
} 