use crate::error::{AppError, Result};
use crate::models::{Directory, DirectoryStructure};
use crate::markdown_generator;
use std::fs;
use std::path::Path;

/// Generates a directory structure based on the provided schema
pub fn generate_structure(
    structure: &DirectoryStructure,
    output_path: &Path,
    organization_name: Option<&str>,
) -> Result<()> {
    // Create the root directory if it doesn't exist
    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    } else if !output_path.is_dir() {
        return Err(AppError::InvalidPath(output_path.to_path_buf()));
    }
    
    // Generate the main README.md
    generate_main_readme(structure, output_path, organization_name)?;
    
    // Generate directories
    for directory in &structure.directories {
        generate_directory(
            structure,
            directory,
            output_path,
            organization_name,
            "",
        )?;
    }
    
    Ok(())
}

/// Generates the main README.md file
fn generate_main_readme(
    structure: &DirectoryStructure,
    output_path: &Path,
    organization_name: Option<&str>,
) -> Result<()> {
    let readme_content = markdown_generator::render_main_readme(structure, organization_name)?;
    let readme_path = output_path.join("README.md");
    
    fs::write(readme_path, readme_content)?;
    
    Ok(())
}

/// Recursively generates a directory and its subdirectories
pub fn generate_directory(
    structure: &DirectoryStructure,
    directory: &Directory,
    parent_path: &Path,
    organization_name: Option<&str>,
    current_path: &str,
) -> Result<()> {
    // Create the directory
    let dir_path = parent_path.join(&directory.name);
    if !dir_path.exists() {
        fs::create_dir(&dir_path)?;
    } else if !dir_path.is_dir() {
        return Err(AppError::InvalidPath(dir_path));
    }
    
    // Calculate the current path for README generation
    let new_path = if current_path.is_empty() {
        directory.name.clone()
    } else {
        format!("{}/{}", current_path, directory.name)
    };
    
    // Generate README.md for this directory
    let readme_content = markdown_generator::render_directory_readme(
        structure,
        directory,
        &new_path,
        organization_name,
    )?;
    let readme_path = dir_path.join("README.md");
    fs::write(readme_path, readme_content)?;
    
    // Generate subdirectories if any
    if let Some(subdirectories) = &directory.subdirectories {
        for subdir in subdirectories {
            generate_directory(
                structure,
                subdir,
                &dir_path,
                organization_name,
                &new_path,
            )?;
        }
    }
    
    Ok(())
}

/// Generates a sample JSON schema file
pub fn generate_sample_schema(output_path: &Path) -> Result<()> {
    let sample_schema = include_str!("../examples/generic-saas/input/sample_schema.json");
    
    if output_path.exists() {
        return Err(AppError::FileExists(output_path.to_path_buf()));
    }
    
    fs::write(output_path, sample_schema)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{DirectoryStructure, Directory};
    use std::collections::HashMap;
    use tempfile::TempDir;
    use std::fs;

    fn create_test_structure() -> DirectoryStructure {
        DirectoryStructure {
            name: "Test Structure".to_string(),
            description: "Test Description".to_string(),
            version: "1.0.0".to_string(),
            last_updated: Some("2023-01-01".to_string()),
            organization: None,
            metadata: None,
            directories: vec![
                Directory {
                    name: "dir1".to_string(),
                    display_name: None,
                    description: "Directory 1".to_string(),
                    purpose: None,
                    priority: None,
                    access_level: None,
                    retention_policy: None,
                    readme_extra: None,
                    allowed_file_types: None,
                    tags: None,
                    workflows: None,
                    additional_info: None,
                    subdirectories: Some(vec![
                        Directory {
                            name: "subdir1".to_string(),
                            display_name: None,
                            description: "Subdirectory 1".to_string(),
                            purpose: None,
                            priority: None,
                            access_level: None,
                            retention_policy: None,
                            readme_extra: None,
                            allowed_file_types: None,
                            tags: None,
                            workflows: None,
                            additional_info: None,
                            subdirectories: None,
                            extra: HashMap::new(),
                        }
                    ]),
                    extra: HashMap::new(),
                },
                Directory {
                    name: "dir2".to_string(),
                    display_name: None,
                    description: "Directory 2".to_string(),
                    purpose: None,
                    priority: None,
                    access_level: None,
                    retention_policy: None,
                    readme_extra: None,
                    allowed_file_types: None,
                    tags: None,
                    workflows: None,
                    additional_info: None,
                    subdirectories: None,
                    extra: HashMap::new(),
                }
            ],
        }
    }

    #[test]
    fn test_generate_structure() {
        let temp_dir = TempDir::new().unwrap();
        let structure = create_test_structure();
        
        let result = generate_structure(&structure, temp_dir.path(), None);
        assert!(result.is_ok());
        
        // Check that the main README.md was created
        let main_readme_path = temp_dir.path().join("README.md");
        assert!(main_readme_path.exists());
        
        // Check that directories were created
        let dir1_path = temp_dir.path().join("dir1");
        let dir2_path = temp_dir.path().join("dir2");
        let subdir1_path = dir1_path.join("subdir1");
        
        assert!(dir1_path.exists() && dir1_path.is_dir());
        assert!(dir2_path.exists() && dir2_path.is_dir());
        assert!(subdir1_path.exists() && subdir1_path.is_dir());
        
        // Check that README.md files were created in each directory
        assert!(dir1_path.join("README.md").exists());
        assert!(dir2_path.join("README.md").exists());
        assert!(subdir1_path.join("README.md").exists());
    }

    #[test]
    fn test_generate_structure_with_organization() {
        let temp_dir = TempDir::new().unwrap();
        let structure = create_test_structure();
        
        let result = generate_structure(&structure, temp_dir.path(), Some("Test Org"));
        assert!(result.is_ok());
        
        // Check that the main README.md contains the organization name
        let main_readme_path = temp_dir.path().join("README.md");
        let content = fs::read_to_string(main_readme_path).unwrap();
        assert!(content.contains("Test Org"));
    }

    #[test]
    fn test_generate_structure_existing_dir() {
        let temp_dir = TempDir::new().unwrap();
        let structure = create_test_structure();
        
        // Create a file with the same name as a directory in our structure
        let file_path = temp_dir.path().join("dir1");
        fs::write(&file_path, "test content").unwrap();
        
        let result = generate_structure(&structure, temp_dir.path(), None);
        assert!(result.is_err());
        
        match result {
            Err(AppError::InvalidPath(_)) => {},
            _ => panic!("Expected InvalidPath error"),
        }
    }

    #[test]
    fn test_generate_sample_schema() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("sample_schema.json");
        
        let result = generate_sample_schema(&output_path);
        assert!(result.is_ok());
        assert!(output_path.exists());
        
        // Test that we can't overwrite an existing file
        let result = generate_sample_schema(&output_path);
        assert!(result.is_err());
        match result {
            Err(AppError::FileExists(_)) => {},
            _ => panic!("Expected FileExists error"),
        }
    }
} 