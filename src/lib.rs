pub mod cli;
pub mod error;
pub mod generator;
pub mod models;
pub mod markdown_generator;
pub mod validator;

// Re-export the main modules for easier access
pub use error::{AppError, Result};
pub use models::DirectoryStructure;
pub use generator::{generate_structure, generate_directory};
pub use validator::validate_schema;
pub use markdown_generator::{render_main_readme, render_directory_readme}; 