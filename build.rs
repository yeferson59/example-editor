use std::process::Command;

fn main() {
    // Print cargo version information
    println!("cargo:rustc-env=RUST_EDITOR_VERSION={}", env!("CARGO_PKG_VERSION"));
    
    // Check for required build dependencies
    check_dependencies();

    // Generate version information
    generate_version_info();
}

fn check_dependencies() {
    // Check for tree-sitter
    if !Command::new("tree-sitter").arg("--version").status().is_ok() {
        println!("cargo:warning=tree-sitter CLI not found. Some features may not work correctly.");
    }

    // Check for npm (needed for some LSP servers)
    if !Command::new("npm").arg("--version").status().is_ok() {
        println!("cargo:warning=npm not found. Some language servers may not be available.");
    }
}

fn generate_version_info() {
    // Get git commit hash if available
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=RUST_EDITOR_GIT_HASH={}", git_hash);

    // Get build timestamp
    let timestamp = chrono::Utc::now().to_rfc3339();
    println!("cargo:rustc-env=RUST_EDITOR_BUILD_TIME={}", timestamp);

    // Get build target
    println!("cargo:rustc-env=RUST_EDITOR_TARGET={}", std::env::var("TARGET").unwrap());

    // Get Rust version
    println!("cargo:rustc-env=RUST_EDITOR_RUST_VERSION={}", rustc_version::version().unwrap());
}
