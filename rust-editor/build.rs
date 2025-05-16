//! Build script for generating build information

fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");
}
