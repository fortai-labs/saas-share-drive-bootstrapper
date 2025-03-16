# aidir

A Rust CLI tool for bootstrapping AI-ready directory structures for startups and organizations based on a standardized schema defined in JSON.

## Introduction

In today's AI-powered workplace, organizing your company's digital assets isn't just about tidiness—it's about unlocking intelligence. The aidir tool creates structured, AI-ready file systems that transform passive storage into active knowledge repositories.

By generating directories with rich metadata and AI-friendly YAML frontmatter, this tool creates an environment where:

- **AI agents can navigate** your company's knowledge base with contextual understanding
- **Intelligent assistants** can locate, reference, and reason about documents based on their purpose, access level, and relationships
- **Knowledge workers** benefit from consistent organization with clear governance and access controls
- **Compliance requirements** are built into the structure through retention policies and access levels

Stop building file systems for humans alone. Create directory structures that work seamlessly with both your team and the AI tools that increasingly power modern workflows.

## Features

- Create a complete directory structure from a JSON schema
- Generate README files for each directory with detailed information and YAML frontmatter metadata
- Include access level badges, usage guidelines, and governance information
- Support for nested directory hierarchies with proper inheritance of properties
- Validate JSON schemas against the schema definition
- Generate sample JSON schemas to get started quickly
- Comprehensive test suite for validating functionality

## Installation

### From Source

1. Clone the repository:

```bash
git clone https://github.com/fortai-labs/aidir.git
cd aidir
```

2. Build the project:

```bash
cargo build --release
```

3. The binary will be available at `target/release/aidir`

## Usage

### Create a Directory Structure

```bash
aidir create --schema path/to/schema.json --output path/to/output/dir --organization "Your Company Name"
```

Options:

- `--schema, -s`: Path to the JSON schema file (required)
- `--output, -o`: Path to the output directory (required)
- `--organization, -o`: Organization name to use in README files (optional)
- `--skip-validation`: Skip validation of the JSON schema (optional)

### Validate a Schema

```bash
aidir validate --schema path/to/schema.json
```

### Generate a Sample Schema

```bash
aidir generate --output path/to/output/schema.json
```

## Schema Format

The schema follows a specific JSON format that defines the directory structure. Here's a simplified example:

```json
{
  "name": "Company_Drive",
  "description": "Centralized repository for all company documents and resources.",
  "version": "1.0.0",
  "last_updated": "2023-03-16T12:00:00Z",
  "organization": "Your Company",
  "metadata": {
    "purpose": "Standardized directory structure for organizing business documents",
    "governance": {
      "owner": "Operations Team",
      "review_cycle": "Quarterly"
    },
    "tags": ["business", "documents", "organization"]
  },
  "directories": [
    {
      "name": "01_Administrative",
      "display_name": "Administrative",
      "description": "Administrative documents including legal, HR, and financial resources.",
      "access_level": "restricted",
      "readme_extra": {
        "usage_guidelines": "Store all administrative documents here.",
        "file_naming_convention": "YYYY-MM-DD_DocumentType_Description.ext",
        "contact_person": {
          "name": "Operations Manager",
          "email": "operations@company.com"
        }
      },
      "subdirectories": [
        {
          "name": "Legal_Documents",
          "display_name": "Legal Documents",
          "description": "Legal documents including contracts and compliance documentation.",
          "access_level": "confidential"
        }
      ]
    }
  ]
}
```

For a complete example, see the [sample schema](examples/generic-saas/input/sample_schema.json).

## Testing

The project includes a comprehensive test suite that validates all aspects of functionality:

```bash
cargo test
```

The test suite includes:

- Unit tests for core functionality
- Integration tests for CLI commands
- Fixture tests using example data
- Schema validation tests
- Markdown generation tests
- Directory regeneration tests

## Generated Output

The tool generates a complete directory structure with README files for each directory. Each README includes:

- YAML frontmatter with metadata (for AI/LLM systems)
- Directory name and description
- Access level with visual badges
- Usage guidelines (when provided)
- File naming conventions (when specified)
- Contact information (when provided)
- Retention policies (when specified)
- A table of subdirectories with descriptions and access levels

### Example Directory Structure

Below is an example of a generated directory structure for a startup:

```
output/
├── README.md                             # Main README with overview and directory listing
├── 01_Administrative/                    # Administrative documents
│   ├── README.md                         # README with department-specific information
│   ├── Legal_Documents/                  # Legal documents
│   │   ├── README.md
│   │   ├── Incorporation_and_Business_Formation/
│   │   │   └── README.md
│   │   ├── Contracts/
│   │   │   └── README.md
│   │   ├── Intellectual_Property/
│   │   │   └── README.md
│   │   └── Compliance/
│   │       └── README.md
│   ├── HR_and_People/                    # HR documents
│   │   ├── README.md
│   │   ├── Employee_Handbook/
│   │   │   └── README.md
│   │   ├── Hiring_and_Recruiting/
│   │   │   └── README.md
│   │   └── ...
│   └── Finance_and_Accounting/           # Financial documents
│       ├── README.md
│       └── ...
├── 02_Product/                           # Product development materials
│   ├── README.md
│   ├── Research/
│   │   └── README.md
│   ├── Product_Strategy/
│   │   └── README.md
│   └── ...
├── 03_Engineering/                       # Engineering resources
│   ├── README.md
│   └── ...
└── ...                                   # Additional directories
```

Each README.md file contains structured information about its directory, making it easy for both humans and AI systems to understand the purpose and organization of the content.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
