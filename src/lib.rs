pub mod cli;
pub mod error;
pub mod generator;
pub mod markdown_generator;
pub mod models;
pub mod validator;

// Re-export the main modules for easier access
pub use error::{AppError, Result};
pub use generator::{generate_directory, generate_structure};
pub use markdown_generator::{render_directory_readme, render_main_readme};
pub use models::DirectoryStructure;
pub use validator::validate_schema;
