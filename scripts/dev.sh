#!/usr/bin/env bash
set -euo pipefail

# Development helper script for Rust Editor

# Command help
show_help() {
    echo "Rust Editor Development Helper"
    echo
    echo "Usage: $0 [command] [options]"
    echo
    echo "Commands:"
    echo "  build        Build the project"
    echo "  test         Run tests"
    echo "  run          Run the editor"
    echo "  check        Run cargo check"
    echo "  lint         Run clippy"
    echo "  fmt          Format code"
    echo "  doc          Generate documentation"
    echo "  clean        Clean build artifacts"
    echo "  plugin       Manage plugins"
    echo
    echo "Options:"
    echo "  --release    Build in release mode"
    echo "  --all        Run on all targets"
    echo "  --verbose    Show verbose output"
    echo
    echo "Examples:"
    echo "  $0 build --release"
    echo "  $0 test --all"
    echo "  $0 plugin build word-count"
}

# Build the project
build() {
    local args=()
    [[ "${RELEASE:-}" == "true" ]] && args+=(--release)
    [[ "${VERBOSE:-}" == "true" ]] && args+=(--verbose)

    echo "Building Rust Editor..."
    cargo build "${args[@]}"
}

# Run tests
run_tests() {
    local args=()
    [[ "${ALL:-}" == "true" ]] && args+=(--workspace --all-features)
    [[ "${RELEASE:-}" == "true" ]] && args+=(--release)
    [[ "${VERBOSE:-}" == "true" ]] && args+=(--verbose)

    echo "Running tests..."
    cargo test "${args[@]}"
}

# Run the editor
run_editor() {
    local args=()
    [[ "${RELEASE:-}" == "true" ]] && args+=(--release)

    echo "Running Rust Editor..."
    cargo run "${args[@]}" -- "$@"
}

# Check the project
check_project() {
    local args=()
    [[ "${ALL:-}" == "true" ]] && args+=(--workspace --all-features)
    [[ "${VERBOSE:-}" == "true" ]] && args+=(--verbose)

    echo "Checking project..."
    cargo check "${args[@]}"
}

# Run clippy
run_lint() {
    local args=()
    [[ "${ALL:-}" == "true" ]] && args+=(--workspace --all-features)
    [[ "${VERBOSE:-}" == "true" ]] && args+=(--verbose)

    echo "Running clippy..."
    cargo clippy "${args[@]}" -- -D warnings
}

# Format code
format_code() {
    echo "Formatting code..."
    cargo fmt --all
}

# Generate documentation
generate_docs() {
    local args=()
    [[ "${VERBOSE:-}" == "true" ]] && args+=(--verbose)

    echo "Generating documentation..."
    cargo doc --no-deps --workspace "${args[@]}"
}

# Clean build artifacts
clean_project() {
    echo "Cleaning project..."
    cargo clean
}

# Manage plugins
manage_plugins() {
    local command=$1
    local plugin_name=$2
    local plugin_dir="examples/plugins/$plugin_name"

    case $command in
        build)
            echo "Building plugin: $plugin_name"
            (cd "$plugin_dir" && cargo build --release)
            ;;
        test)
            echo "Testing plugin: $plugin_name"
            (cd "$plugin_dir" && cargo test)
            ;;
        install)
            echo "Installing plugin: $plugin_name"
            mkdir -p ~/.config/rust-editor/plugins/"$plugin_name"
            cp "$plugin_dir/target/release/lib${plugin_name}_plugin".* ~/.config/rust-editor/plugins/"$plugin_name"/
            cp "$plugin_dir/plugin.json" ~/.config/rust-editor/plugins/"$plugin_name"/
            ;;
        *)
            echo "Unknown plugin command: $command"
            exit 1
            ;;
    esac
}

# Parse command line arguments
COMMAND=""
RELEASE=false
ALL=false
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        build|test|run|check|lint|fmt|doc|clean|plugin)
            COMMAND=$1
            shift
            ;;
        --release)
            RELEASE=true
            shift
            ;;
        --all)
            ALL=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            break
            ;;
    esac
done

# Execute command
case $COMMAND in
    build)
        build
        ;;
    test)
        run_tests
        ;;
    run)
        run_editor "$@"
        ;;
    check)
        check_project
        ;;
    lint)
        run_lint
        ;;
    fmt)
        format_code
        ;;
    doc)
        generate_docs
        ;;
    clean)
        clean_project
        ;;
    plugin)
        manage_plugins "$@"
        ;;
    *)
        show_help
        exit 1
        ;;
esac
