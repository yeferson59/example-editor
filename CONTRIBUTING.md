# Contributing to Rust Editor

We love your input! We want to make contributing to Rust Editor as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## Development Process

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests.

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code lints.
6. Issue that pull request!

## Development Setup

1. Install Rust and Cargo:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install dependencies:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install libx11-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

   # macOS
   brew install tree-sitter

   # Windows
   # Install Visual Studio Build Tools
   ```

3. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rust-editor.git
   cd rust-editor
   ```

4. Build the project:
   ```bash
   cargo build
   ```

## Using Docker for Development

We provide a Docker development environment:

```bash
# Start development environment
docker-compose up dev

# Run tests
docker-compose run test
```

## Development Commands

Use the development script for common tasks:

```bash
# Build the project
./scripts/dev.sh build

# Run tests
./scripts/dev.sh test

# Format code
./scripts/dev.sh fmt

# Check code
./scripts/dev.sh check

# Run linter
./scripts/dev.sh lint

# Generate docs
./scripts/dev.sh doc

# Build and install a plugin
./scripts/dev.sh plugin build word-count
./scripts/dev.sh plugin install word-count
```

## Pull Request Process

1. Update the README.md with details of changes to the interface.
2. Update the CHANGELOG.md with a note describing your changes.
3. Update any relevant documentation.
4. The PR will be merged once you have the sign-off of two other developers.

## Any contributions you make will be under the MIT Software License

In short, when you submit code changes, your submissions are understood to be under the same [MIT License](http://choosealicense.com/licenses/mit/) that covers the project. Feel free to contact the maintainers if that's a concern.

## Report bugs using GitHub's [issue tracker](https://github.com/yourusername/rust-editor/issues)

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/yourusername/rust-editor/issues/new); it's that easy!

## Write bug reports with detail, background, and sample code

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can.
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

## License

By contributing, you agree that your contributions will be licensed under its MIT License.

## References

This document was adapted from the open-source contribution guidelines for [Facebook's Draft](https://github.com/facebook/draft-js/blob/a9316a723f9e918afde44dea68b5f9f39b7d9b00/CONTRIBUTING.md).
