# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-03-16

### Added

- Initial release of the project
- Core functionality for generating directory structures from JSON schemas
- Markdown generation for README files with YAML frontmatter
- Schema validation against JSON schema
- CLI interface with create, validate, and generate commands
- Comprehensive test suite including:
  - Unit tests for core functionality
  - Integration tests for CLI commands
  - Fixture tests using example data
  - Schema validation tests
  - Markdown generation tests
  - Directory regeneration tests
- Example schema and output for a generic startup directory structure

### Changed

- Renamed project from "SaaS Share Drive Bootstrapper" to "aidir" to reflect its more generic use case
- Updated all documentation and code references to use the new name
- Refactored the CLI interface to use the new name

### Fixed

- Various test failures related to path mismatches in fixture tests
- Corrected assertions in markdown tests to match actual output format

## [0.0.1] - 2024-03-15

### Added

- Initial project setup
- Basic directory structure generation
- JSON schema parsing and validation
