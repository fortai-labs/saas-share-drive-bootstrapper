#!/bin/bash
set -e

# Check if a version is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <version>"
  echo "Example: $0 0.2.0"
  exit 1
fi

VERSION=$1
DATE=$(date +%Y-%m-%d)

# Update version in Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml && rm Cargo.toml.bak

# Add new version to CHANGELOG.md
awk -v version="$VERSION" -v date="$DATE" '
  /^## / { if (!found) { print "## [" version "] - " date "\n"; print "### Added\n- \n"; print "### Changed\n- \n"; print "### Fixed\n- \n"; found=1; } }
  { print }
' CHANGELOG.md > CHANGELOG.md.new && mv CHANGELOG.md.new CHANGELOG.md

echo "Updated version to $VERSION in Cargo.toml and CHANGELOG.md"
echo "Please edit CHANGELOG.md to add your changes, then run:"
echo "git add Cargo.toml CHANGELOG.md"
echo "git commit -m \"Release v$VERSION\""
echo "git tag v$VERSION"
echo "git push && git push --tags" 