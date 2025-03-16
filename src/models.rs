use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root structure representing the entire directory structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectoryStructure {
    pub name: String,
    pub description: String,
    pub version: String,
    #[serde(default)]
    pub last_updated: Option<String>,
    #[serde(default)]
    pub organization: Option<String>,
    #[serde(default)]
    pub metadata: Option<Metadata>,
    pub directories: Vec<Directory>,
}

/// Metadata about the directory structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    #[serde(default)]
    pub purpose: Option<String>,
    #[serde(default)]
    pub governance: Option<Governance>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Governance information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Governance {
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub review_cycle: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Directory entry with all its properties
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Directory {
    pub name: String,
    #[serde(default)]
    pub display_name: Option<String>,
    pub description: String,
    #[serde(default)]
    pub purpose: Option<String>,
    #[serde(default)]
    pub priority: Option<u8>,
    #[serde(default)]
    pub access_level: Option<String>,
    #[serde(default)]
    pub retention_policy: Option<String>,
    #[serde(default)]
    pub readme_extra: Option<ReadmeExtra>,
    #[serde(default)]
    pub allowed_file_types: Option<Vec<String>>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub workflows: Option<Vec<Workflow>>,
    #[serde(default)]
    pub additional_info: Option<String>,
    #[serde(default)]
    pub subdirectories: Option<Vec<Directory>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Additional README content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReadmeExtra {
    #[serde(default)]
    pub usage_guidelines: Option<String>,
    #[serde(default)]
    pub file_naming_convention: Option<String>,
    #[serde(default)]
    pub examples: Option<Vec<Example>>,
    #[serde(default)]
    pub related_resources: Option<Vec<RelatedResource>>,
    #[serde(default)]
    pub contact_person: Option<ContactPerson>,
    #[serde(default)]
    pub faq: Option<Vec<Faq>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Example files or use cases
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Example {
    pub description: String,
    pub example: String,
}

/// Links to related resources
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelatedResource {
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// Contact person information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactPerson {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub slack_channel: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// FAQ entries
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Faq {
    pub question: String,
    pub answer: String,
}

/// Business workflows
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workflow {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub steps: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
} 