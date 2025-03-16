# Scripts

This directory contains utility scripts for the aidir project.

## Release Script

The `release.sh` script helps with creating new releases by:

1. Updating the version in `Cargo.toml`
2. Adding a new version section to `CHANGELOG.md`
3. Providing instructions for creating a git tag and pushing changes

### Usage

```bash
./scripts/release.sh <version>
```

Example:

```bash
./scripts/release.sh 0.2.0
```

After running the script:

1. Edit the CHANGELOG.md to add your changes
2. Commit and tag the release
3. Push the changes and tags

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Release v0.2.0"
git tag v0.2.0
git push && git push --tags
```
