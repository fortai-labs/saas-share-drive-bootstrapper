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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_value, json};

    #[test]
    fn test_directory_structure_serialization() {
        let structure = DirectoryStructure {
            name: "Test Structure".to_string(),
            description: "Test Description".to_string(),
            version: "1.0.0".to_string(),
            last_updated: Some("2023-01-01".to_string()),
            organization: Some("Test Org".to_string()),
            metadata: None,
            directories: vec![Directory {
                name: "test-dir".to_string(),
                display_name: None,
                description: "Test Directory".to_string(),
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
            }],
        };

        let json = serde_json::to_string(&structure).unwrap();
        let deserialized: DirectoryStructure = serde_json::from_str(&json).unwrap();

        assert_eq!(structure.name, deserialized.name);
        assert_eq!(structure.description, deserialized.description);
        assert_eq!(structure.version, deserialized.version);
        assert_eq!(structure.last_updated, deserialized.last_updated);
        assert_eq!(structure.organization, deserialized.organization);
        assert_eq!(structure.directories.len(), deserialized.directories.len());
        assert_eq!(
            structure.directories[0].name,
            deserialized.directories[0].name
        );
        assert_eq!(
            structure.directories[0].description,
            deserialized.directories[0].description
        );
    }

    #[test]
    fn test_directory_with_subdirectories() {
        let json_value = json!({
            "name": "parent",
            "description": "Parent directory",
            "subdirectories": [
                {
                    "name": "child1",
                    "description": "Child directory 1"
                },
                {
                    "name": "child2",
                    "description": "Child directory 2"
                }
            ]
        });

        let directory: Directory = from_value(json_value).unwrap();

        assert_eq!(directory.name, "parent");
        assert_eq!(directory.description, "Parent directory");
        assert!(directory.subdirectories.is_some());

        let subdirs = directory.subdirectories.unwrap();
        assert_eq!(subdirs.len(), 2);
        assert_eq!(subdirs[0].name, "child1");
        assert_eq!(subdirs[1].name, "child2");
    }

    #[test]
    fn test_directory_with_readme_extra() {
        let json_value = json!({
            "name": "test-dir",
            "description": "Test directory",
            "readme_extra": {
                "usage_guidelines": "Use this for important files",
                "file_naming_convention": "prefix_name_date.ext",
                "examples": [
                    {
                        "description": "Example file",
                        "example": "example_file_2023.txt"
                    }
                ],
                "faq": [
                    {
                        "question": "What goes here?",
                        "answer": "Important files"
                    }
                ]
            }
        });

        let directory: Directory = from_value(json_value).unwrap();

        assert_eq!(directory.name, "test-dir");
        assert!(directory.readme_extra.is_some());

        let readme = directory.readme_extra.unwrap();
        assert_eq!(
            readme.usage_guidelines,
            Some("Use this for important files".to_string())
        );
        assert_eq!(
            readme.file_naming_convention,
            Some("prefix_name_date.ext".to_string())
        );

        assert!(readme.examples.is_some());
        let examples = readme.examples.unwrap();
        assert_eq!(examples.len(), 1);
        assert_eq!(examples[0].description, "Example file");

        assert!(readme.faq.is_some());
        let faqs = readme.faq.unwrap();
        assert_eq!(faqs.len(), 1);
        assert_eq!(faqs[0].question, "What goes here?");
        assert_eq!(faqs[0].answer, "Important files");
    }

    #[test]
    fn test_metadata_with_governance() {
        let json_value = json!({
            "purpose": "Test purpose",
            "governance": {
                "owner": "Test Owner",
                "review_cycle": "Quarterly"
            },
            "tags": ["test", "example"]
        });

        let metadata: Metadata = from_value(json_value).unwrap();

        assert_eq!(metadata.purpose, Some("Test purpose".to_string()));
        assert!(metadata.governance.is_some());
        assert!(metadata.tags.is_some());

        let governance = metadata.governance.unwrap();
        assert_eq!(governance.owner, Some("Test Owner".to_string()));
        assert_eq!(governance.review_cycle, Some("Quarterly".to_string()));

        let tags = metadata.tags.unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0], "test");
        assert_eq!(tags[1], "example");
    }

    #[test]
    fn test_extra_fields_handling() {
        let json_value = json!({
            "name": "test-dir",
            "description": "Test directory",
            "custom_field": "custom value",
            "another_custom": 123
        });

        let directory: Directory = from_value(json_value).unwrap();

        assert_eq!(directory.name, "test-dir");
        assert_eq!(directory.description, "Test directory");
        assert!(directory.extra.contains_key("custom_field"));
        assert!(directory.extra.contains_key("another_custom"));

        if let Some(value) = directory.extra.get("custom_field") {
            assert_eq!(value.as_str().unwrap(), "custom value");
        } else {
            panic!("Expected custom_field to exist");
        }

        if let Some(value) = directory.extra.get("another_custom") {
            assert_eq!(value.as_i64().unwrap(), 123);
        } else {
            panic!("Expected another_custom to exist");
        }
    }
}
