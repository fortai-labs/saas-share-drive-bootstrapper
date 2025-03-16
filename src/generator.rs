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
fn generate_directory(
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