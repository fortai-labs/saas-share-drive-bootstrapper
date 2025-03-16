use crate::error::Result;
use crate::models::{Directory, DirectoryStructure};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Root directory frontmatter structure
#[derive(Serialize, Deserialize)]
struct RootFrontmatter {
    structure_type: String,
    name: String,
    organization: String,
    version: String,
    last_updated: Option<String>,
    purpose: Option<String>,
    governance: Option<GovernanceInfo>,
    tags: Option<Vec<String>>,
    top_level_directories: Vec<DirectoryRef>,
    generated_date: String,
}

/// Directory frontmatter structure
#[derive(Serialize, Deserialize)]
struct DirectoryFrontmatter {
    structure_type: String,
    name: String,
    display_name: String,
    description: String,
    path: String,
    parent_structure: String,
    organization: String,
    access_level: String,
    priority: Option<String>,
    retention_policy: Option<String>,
    allowed_file_types: Option<Vec<String>>,
    additional_info: Option<String>,
    subdirectories: Option<Vec<DirectoryRef>>,
    last_updated: String,
}

/// Governance information for frontmatter
#[derive(Serialize, Deserialize)]
struct GovernanceInfo {
    owner: Option<String>,
    review_cycle: Option<String>,
}

/// Directory reference for frontmatter
#[derive(Serialize, Deserialize)]
struct DirectoryRef {
    name: String,
    display_name: String,
    description: String,
    access_level: String,
    path: String,
}

/// Generates an access level badge for markdown
fn access_level_badge(access_level: &str) -> String {
    let (color, text) = match access_level.to_lowercase().as_str() {
        "public" => ("brightgreen", "Public"),
        "team" => ("blue", "Team"),
        "restricted" => ("yellow", "Restricted"),
        "confidential" => ("red", "Confidential"),
        _ => ("lightgrey", access_level),
    };

    format!(
        "![Access: {}](https://img.shields.io/badge/Access-{}-{})",
        text, text, color
    )
}

/// Formats a date in a nice way
fn format_date(date_str: &str) -> String {
    if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
        date.format("%B %d, %Y").to_string()
    } else {
        date_str.to_string()
    }
}

/// Renders the main README.md file for the root directory
pub fn render_main_readme(
    structure: &DirectoryStructure,
    organization_name: Option<&str>,
) -> Result<String> {
    let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let org_name = organization_name.unwrap_or(structure.organization.as_deref().unwrap_or(""));

    // Create top-level directory references
    let top_dirs: Vec<DirectoryRef> = structure
        .directories
        .iter()
        .map(|dir| DirectoryRef {
            name: dir.name.clone(),
            display_name: dir.display_name.clone().unwrap_or_else(|| dir.name.clone()),
            description: dir.description.clone(),
            access_level: dir
                .access_level
                .clone()
                .unwrap_or_else(|| "team".to_string()),
            path: format!("./{}", dir.name),
        })
        .collect();

    // Create frontmatter
    let frontmatter = RootFrontmatter {
        structure_type: "root_directory".to_string(),
        name: structure.name.clone(),
        organization: org_name.to_string(),
        version: structure.version.clone(),
        last_updated: structure.last_updated.clone(),
        purpose: structure.metadata.as_ref().and_then(|m| m.purpose.clone()),
        governance: structure.metadata.as_ref().and_then(|m| {
            m.governance.as_ref().map(|g| GovernanceInfo {
                owner: g.owner.clone(),
                review_cycle: g.review_cycle.clone(),
            })
        }),
        tags: structure.metadata.as_ref().and_then(|m| m.tags.clone()),
        top_level_directories: top_dirs,
        generated_date: current_date.clone(),
    };

    // Serialize frontmatter to YAML
    let yaml_frontmatter = serde_yaml::to_string(&frontmatter)?;

    // Build the markdown content
    let mut content = String::new();

    // Add YAML frontmatter
    writeln!(content, "---\n{}---\n", yaml_frontmatter)?;

    // Add title and description
    writeln!(content, "# {}", structure.name)?;
    writeln!(content, "\n{}\n", structure.description)?;

    // Add overview section
    writeln!(content, "## Overview\n")?;
    writeln!(
        content,
        "This directory structure was generated on {} using the SaaS Share Drive Bootstrapper tool.\n",
        current_date
    )?;
    writeln!(content, "**Organization:** {}  ", org_name)?;
    writeln!(content, "**Version:** {}  ", structure.version)?;

    if let Some(last_updated) = &structure.last_updated {
        writeln!(content, "**Last Updated:** {}  ", format_date(last_updated))?;
    }

    // Add purpose section if available
    if let Some(metadata) = &structure.metadata {
        if let Some(purpose) = &metadata.purpose {
            writeln!(content, "\n## Purpose\n")?;
            writeln!(content, "{}\n", purpose)?;
        }
    }

    // Add directory structure section
    writeln!(content, "## Directory Structure\n")?;
    writeln!(content, "| Directory | Description | Access Level |")?;
    writeln!(content, "|-----------|-------------|--------------|")?;

    for dir in &structure.directories {
        let display_name = dir.display_name.as_deref().unwrap_or(&dir.name);
        let access_level = dir.access_level.as_deref().unwrap_or("team");

        writeln!(
            content,
            "| [{}](./{}/) | {} | {} |",
            display_name,
            dir.name,
            dir.description,
            access_level_badge(access_level)
        )?;
    }

    // Add governance section if available
    if let Some(metadata) = &structure.metadata {
        if let Some(governance) = &metadata.governance {
            writeln!(content, "\n## Governance\n")?;

            if let Some(owner) = &governance.owner {
                writeln!(content, "**Owner:** {}  ", owner)?;
            }

            if let Some(review_cycle) = &governance.review_cycle {
                writeln!(content, "**Review Cycle:** {}  ", review_cycle)?;
            }
        }
    }

    // Add tags section if available
    if let Some(metadata) = &structure.metadata {
        if let Some(tags) = &metadata.tags {
            if !tags.is_empty() {
                writeln!(content, "\n## Tags\n")?;

                for tag in tags {
                    writeln!(content, "`{}`  ", tag)?;
                }
            }
        }
    }

    // Add footer
    writeln!(content, "\n---\n")?;
    writeln!(
        content,
        "*This directory structure is maintained according to the organization's data governance policies.*"
    )?;

    Ok(content)
}

/// Renders a README.md file for a specific directory
pub fn render_directory_readme(
    structure: &DirectoryStructure,
    directory: &Directory,
    path: &str,
    organization_name: Option<&str>,
) -> Result<String> {
    let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let org_name = organization_name.unwrap_or(structure.organization.as_deref().unwrap_or(""));

    // Create subdirectory references if they exist
    let subdirs = directory.subdirectories.as_ref().map(|subdirs| {
        subdirs
            .iter()
            .map(|dir| DirectoryRef {
                name: dir.name.clone(),
                display_name: dir.display_name.clone().unwrap_or_else(|| dir.name.clone()),
                description: dir.description.clone(),
                access_level: dir
                    .access_level
                    .clone()
                    .unwrap_or_else(|| "team".to_string()),
                path: format!("./{}", dir.name),
            })
            .collect::<Vec<_>>()
    });

    // Create frontmatter
    let frontmatter = DirectoryFrontmatter {
        structure_type: "directory".to_string(),
        name: directory.name.clone(),
        display_name: directory
            .display_name
            .clone()
            .unwrap_or_else(|| directory.name.clone()),
        description: directory.description.clone(),
        path: path.to_string(),
        parent_structure: structure.name.clone(),
        organization: org_name.to_string(),
        access_level: directory
            .access_level
            .clone()
            .unwrap_or_else(|| "team".to_string()),
        priority: directory.priority.as_ref().map(|p| p.to_string()),
        retention_policy: directory.retention_policy.clone(),
        allowed_file_types: directory.allowed_file_types.clone(),
        additional_info: directory.additional_info.clone(),
        subdirectories: subdirs,
        last_updated: current_date.clone(),
    };

    // Serialize frontmatter to YAML
    let yaml_frontmatter = serde_yaml::to_string(&frontmatter)?;

    // Build the markdown content
    let mut content = String::new();

    // Add YAML frontmatter
    writeln!(content, "---\n{}---\n", yaml_frontmatter)?;

    // Add title and description
    let display_name = directory.display_name.as_deref().unwrap_or(&directory.name);
    writeln!(content, "# {}", display_name)?;
    writeln!(content, "\n{}\n", directory.description)?;

    // Add overview section
    writeln!(content, "## Overview\n")?;

    let access_level = directory.access_level.as_deref().unwrap_or("team");
    writeln!(
        content,
        "**Access Level:** {}  ",
        access_level_badge(access_level)
    )?;

    if let Some(priority) = &directory.priority {
        writeln!(content, "**Priority:** {}  ", priority)?;
    }

    writeln!(content, "**Last Updated:** {}  ", current_date)?;

    // Add usage guidelines if available
    if let Some(readme_extra) = &directory.readme_extra {
        if let Some(usage_guidelines) = &readme_extra.usage_guidelines {
            writeln!(content, "\n## Usage Guidelines\n")?;
            writeln!(content, "{}\n", usage_guidelines)?;
        }

        // Add file naming convention if available
        if let Some(file_naming_convention) = &readme_extra.file_naming_convention {
            writeln!(content, "\n## File Naming Convention\n")?;
            writeln!(content, "`{}`\n", file_naming_convention)?;
        }

        // Add contact information if available
        if let Some(contact_person) = &readme_extra.contact_person {
            writeln!(content, "\n## Contact Information\n")?;

            if let Some(name) = &contact_person.name {
                writeln!(content, "**Contact:** {}  ", name)?;
            }

            if let Some(email) = &contact_person.email {
                writeln!(content, "**Email:** {}  ", email)?;
            }

            if let Some(slack) = &contact_person.slack_channel {
                writeln!(content, "**Slack:** {}  ", slack)?;
            }
        }
    }

    // Add retention policy if available
    if let Some(retention_policy) = &directory.retention_policy {
        writeln!(content, "\n## Retention Policy\n")?;
        writeln!(content, "{}\n", retention_policy)?;
    }

    // Add allowed file types if available
    if let Some(allowed_file_types) = &directory.allowed_file_types {
        if !allowed_file_types.is_empty() {
            writeln!(content, "\n## Allowed File Types\n")?;

            for file_type in allowed_file_types {
                writeln!(content, "- `{}`", file_type)?;
            }
        }
    }

    // Add subdirectories section if available
    if let Some(subdirectories) = &directory.subdirectories {
        if !subdirectories.is_empty() {
            writeln!(content, "\n## Subdirectories\n")?;
            writeln!(content, "| Directory | Description | Access Level |")?;
            writeln!(content, "|-----------|-------------|--------------|")?;

            for subdir in subdirectories {
                let display_name = subdir.display_name.as_deref().unwrap_or(&subdir.name);
                let access_level = subdir.access_level.as_deref().unwrap_or("team");

                writeln!(
                    content,
                    "| [{}](./{}/) | {} | {} |",
                    display_name,
                    subdir.name,
                    subdir.description,
                    access_level_badge(access_level)
                )?;
            }
        }
    }

    // Add additional information if available
    if let Some(additional_info) = &directory.additional_info {
        writeln!(content, "\n## Additional Information\n")?;
        writeln!(content, "{}\n", additional_info)?;
    }

    Ok(content)
}
