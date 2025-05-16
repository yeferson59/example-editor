#!/usr/bin/env bash
set -euo pipefail

# Script to manage releases for Rust Editor

# Show help
show_help() {
    echo "Rust Editor Release Manager"
    echo
    echo "Usage: $0 [command] [version]"
    echo
    echo "Commands:"
    echo "  bump        Bump version number"
    echo "  prepare     Prepare release"
    echo "  publish     Publish release"
    echo "  changelog   Update changelog"
    echo
    echo "Options:"
    echo "  major       Bump major version"
    echo "  minor       Bump minor version"
    echo "  patch       Bump patch version"
    echo
    echo "Examples:"
    echo "  $0 bump minor"
    echo "  $0 prepare 1.2.0"
    echo "  $0 publish"
}

# Get current version
get_version() {
    cargo pkgid | cut -d# -f2 | cut -d: -f2
}

# Bump version
bump_version() {
    local current_version=$(get_version)
    local version_parts=(${current_version//./ })
    local major=${version_parts[0]}
    local minor=${version_parts[1]}
    local patch=${version_parts[2]}

    case $1 in
        major)
            echo "$((major + 1)).0.0"
            ;;
        minor)
            echo "${major}.$((minor + 1)).0"
            ;;
        patch)
            echo "${major}.${minor}.$((patch + 1))"
            ;;
        *)
            echo "Invalid version type: $1"
            exit 1
            ;;
    esac
}

# Update version in Cargo.toml files
update_versions() {
    local new_version=$1
    
    # Update workspace version
    sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    
    # Update all crate versions
    for toml in */Cargo.toml; do
        sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" "$toml"
    done
    
    # Clean up backup files
    find . -name "*.bak" -delete
}

# Update changelog
update_changelog() {
    local version=$1
    local date=$(date +%Y-%m-%d)
    
    # Create new changelog entry
    local temp_file=$(mktemp)
    echo "## [$version] - $date" > "$temp_file"
    echo "" >> "$temp_file"
    echo "### Added" >> "$temp_file"
    echo "- " >> "$temp_file"
    echo "" >> "$temp_file"
    echo "### Changed" >> "$temp_file"
    echo "- " >> "$temp_file"
    echo "" >> "$temp_file"
    echo "### Fixed" >> "$temp_file"
    echo "- " >> "$temp_file"
    echo "" >> "$temp_file"
    
    # Prepend to CHANGELOG.md
    sed -i.bak "4r $temp_file" CHANGELOG.md
    rm "$temp_file"
    rm CHANGELOG.md.bak
}

# Prepare release
prepare_release() {
    local version=$1
    
    echo "Preparing release $version..."
    
    # Update versions
    update_versions "$version"
    
    # Update changelog
    update_changelog "$version"
    
    # Create git commit
    git add Cargo.toml */Cargo.toml CHANGELOG.md
    git commit -m "Release version $version"
    
    # Create git tag
    git tag -a "v$version" -m "Version $version"
    
    echo "Release $version prepared. Review changes and run '$0 publish' to publish."
}

# Publish release
publish_release() {
    echo "Publishing release..."
    
    # Push changes and tags
    git push
    git push --tags
    
    # Publish to crates.io
    cargo publish --workspace
}

# Parse command line arguments
COMMAND=""
VERSION=""

while [[ $# -gt 0 ]]; do
    case $1 in
        bump|prepare|publish|changelog)
            COMMAND=$1
            shift
            ;;
        major|minor|patch)
            VERSION=$(bump_version $1)
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            VERSION=$1
            shift
            ;;
    esac
done

# Execute command
case $COMMAND in
    bump)
        if [[ -z "$VERSION" ]]; then
            echo "Error: Version type required (major, minor, or patch)"
            exit 1
        fi
        echo "New version: $VERSION"
        ;;
    prepare)
        if [[ -z "$VERSION" ]]; then
            echo "Error: Version number required"
            exit 1
        fi
        prepare_release "$VERSION"
        ;;
    publish)
        publish_release
        ;;
    changelog)
        if [[ -z "$VERSION" ]]; then
            VERSION=$(get_version)
        fi
        update_changelog "$VERSION"
        ;;
    *)
        show_help
        exit 1
        ;;
esac
