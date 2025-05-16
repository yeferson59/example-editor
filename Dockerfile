# Development environment for Rust Editor
FROM rust:1.70-bullseye

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libx11-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    tree-sitter-cli \
    nodejs \
    npm \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install development tools
RUN cargo install cargo-watch cargo-edit cargo-audit

# Set up development directory
WORKDIR /workspace

# Copy project files
COPY . .

# Build dependencies
RUN cargo build

# Set up development environment
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1

# Command to run development shell
CMD ["bash"]
