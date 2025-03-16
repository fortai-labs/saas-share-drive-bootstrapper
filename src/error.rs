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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_io_error_conversion() {
        let io_error = IoError::new(ErrorKind::NotFound, "file not found");
        let app_error: AppError = io_error.into();
        
        match app_error {
            AppError::Io(_) => {},
            _ => panic!("Expected Io error variant"),
        }
        
        let error_message = format!("{}", app_error);
        assert!(error_message.contains("IO error"));
    }

    #[test]
    fn test_json_error_conversion() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let app_error: AppError = json_error.into();
        
        match app_error {
            AppError::Json(_) => {},
            _ => panic!("Expected Json error variant"),
        }
        
        let error_message = format!("{}", app_error);
        assert!(error_message.contains("JSON error"));
    }

    #[test]
    fn test_schema_validation_error() {
        let app_error = AppError::SchemaValidation("Invalid schema format".to_string());
        let error_message = format!("{}", app_error);
        
        assert!(error_message.contains("JSON Schema validation error"));
        assert!(error_message.contains("Invalid schema format"));
    }

    #[test]
    fn test_file_exists_error() {
        let path = PathBuf::from("/path/to/file.txt");
        let app_error = AppError::FileExists(path.clone());
        let error_message = format!("{}", app_error);
        
        assert!(error_message.contains("File already exists"));
        assert!(error_message.contains(path.to_string_lossy().as_ref()));
    }

    #[test]
    fn test_invalid_schema_error() {
        let app_error = AppError::InvalidSchema("Missing required field".to_string());
        let error_message = format!("{}", app_error);
        
        assert!(error_message.contains("Invalid schema"));
        assert!(error_message.contains("Missing required field"));
    }

    #[test]
    fn test_missing_field_error() {
        let app_error = AppError::MissingField("name".to_string());
        let error_message = format!("{}", app_error);
        
        assert!(error_message.contains("Missing required field"));
        assert!(error_message.contains("name"));
    }

    #[test]
    fn test_invalid_path_error() {
        let path = PathBuf::from("/invalid/path");
        let app_error = AppError::InvalidPath(path.clone());
        let error_message = format!("{}", app_error);
        
        assert!(error_message.contains("Invalid path"));
        assert!(error_message.contains(path.to_string_lossy().as_ref()));
    }

    #[test]
    fn test_template_error() {
        let app_error = AppError::Template("Template error".to_string());
        let error_message = format!("{}", app_error);
        
        assert!(error_message.contains("Template rendering error"));
        assert!(error_message.contains("Template error"));
    }

    #[test]
    fn test_directory_exists_error() {
        let path = PathBuf::from("/path/to/dir");
        let app_error = AppError::DirectoryExists(path.clone());
        let error_message = format!("{}", app_error);
        
        assert!(error_message.contains("Directory already exists"));
        assert!(error_message.contains(path.to_string_lossy().as_ref()));
    }

    #[test]
    fn test_unknown_error() {
        let app_error = AppError::Unknown("Unknown error".to_string());
        let error_message = format!("{}", app_error);
        
        assert!(error_message.contains("Unknown error"));
    }

    #[test]
    fn test_result_type() {
        let success_result: Result<i32> = Ok(42);
        let error_result: Result<i32> = Err(AppError::Unknown("test error".to_string()));
        
        assert!(success_result.is_ok());
        assert!(error_result.is_err());
        
        assert_eq!(success_result.unwrap(), 42);
        
        match error_result {
            Err(AppError::Unknown(msg)) => assert_eq!(msg, "test error"),
            _ => panic!("Expected Unknown error variant"),
        }
    }
} 