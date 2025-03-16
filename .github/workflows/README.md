# GitHub Actions Workflows

This directory contains GitHub Actions workflows for the aidir project.

## CI Workflow

The `ci.yml` workflow runs on pull requests to the main branch and when code is pushed to main. It performs the following checks:

1. **Linting**: Runs `rustfmt` and `clippy` to ensure code quality and style
2. **Testing**: Runs all tests to verify functionality
3. **Security Audit**: Uses `cargo-audit` to check for security vulnerabilities in dependencies
4. **Code Coverage**: Uses `cargo-tarpaulin` to generate code coverage reports and uploads them to Codecov

## Release Workflow

The `release.yml` workflow runs when a new tag with the format `v*.*.*` is pushed. It performs the following tasks:

1. **Building**: Builds the binary for multiple platforms:
   - Linux (x86_64)
   - macOS (x86_64)
   - Windows (x86_64)
2. **Packaging**: Creates compressed archives of the binaries
3. **Publishing**: Uploads the archives to the GitHub release and publishes the package to crates.io

## Creating a Release

To create a new release:

1. Update the version in `Cargo.toml` and `CHANGELOG.md` (you can use `./scripts/release.sh <version>`)
2. Commit the changes and push them
3. Create and push a new tag with the format `v*.*.*`

```bash
git tag v0.1.0
git push origin v0.1.0
```

The release workflow will automatically build the binaries and publish them to GitHub Releases and crates.io.
