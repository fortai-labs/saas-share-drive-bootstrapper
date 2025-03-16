use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Format error: {0}")]
    Format(#[from] std::fmt::Error),

    #[error("JSON Schema validation error: {0}")]
    SchemaValidation(String),

    #[error("Template rendering error: {0}")]
    Template(String),

    #[error("File already exists: {0}")]
    FileExists(PathBuf),

    #[error("Directory already exists: {0}")]
    DirectoryExists(PathBuf),

    #[error("Invalid schema: {0}")]
    InvalidSchema(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, AppError>; 