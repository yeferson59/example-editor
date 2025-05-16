//! Benchmarks for Rust Editor
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use editor_core::{Buffer, Document};
use editor_syntax::Highlighter;
use editor_plugin::{PluginManager, testing::MockPlugin};
use std::time::Duration;

/// Benchmarks text operations
fn bench_text_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_operations");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    // Benchmark small insertions
    group.bench_function("small_insert", |b| {
        b.iter_with_setup(
            || Buffer::new(),
            |mut buffer| {
                buffer.insert(0, "Hello, World!").unwrap();
            },
        )
    });

    // Benchmark large insertions
    let large_text = "Hello, World!".repeat(1000);
    group.bench_function("large_insert", |b| {
        b.iter_with_setup(
            || Buffer::new(),
            |mut buffer| {
                buffer.insert(0, &large_text).unwrap();
            },
        )
    });

    // Benchmark deletions
    group.bench_function("delete", |b| {
        b.iter_with_setup(
            || {
                let mut buffer = Buffer::new();
                buffer.insert(0, &large_text).unwrap();
                buffer
            },
            |mut buffer| {
                buffer.delete(0, 100).unwrap();
            },
        )
    });

    // Benchmark search operations
    group.bench_function("search", |b| {
        b.iter_with_setup(
            || {
                let mut buffer = Buffer::new();
                buffer.insert(0, &large_text).unwrap();
                buffer
            },
            |buffer| {
                buffer.search("World");
            },
        )
    });

    group.finish();
}

/// Benchmarks syntax highlighting
fn bench_syntax_highlighting(c: &mut Criterion) {
    let mut group = c.benchmark_group("syntax_highlighting");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    // Test content
    let rust_code = r#"
        fn main() {
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(i);
            }
            println!("Sum: {}", vec.iter().sum::<i32>());
        }
    "#;

    let large_rust_code = rust_code.repeat(100);

    // Benchmark small file highlighting
    group.bench_function("small_file", |b| {
        b.iter_with_setup(
            || {
                let mut highlighter = Highlighter::new();
                highlighter.set_language("rust").unwrap();
                (highlighter, rust_code.to_string())
            },
            |(highlighter, code)| {
                highlighter.highlight(&code).unwrap();
            },
        )
    });

    // Benchmark large file highlighting
    group.bench_function("large_file", |b| {
        b.iter_with_setup(
            || {
                let mut highlighter = Highlighter::new();
                highlighter.set_language("rust").unwrap();
                (highlighter, large_rust_code.to_string())
            },
            |(highlighter, code)| {
                highlighter.highlight(&code).unwrap();
            },
        )
    });

    // Benchmark incremental highlighting
    group.bench_function("incremental", |b| {
        b.iter_with_setup(
            || {
                let mut highlighter = Highlighter::new();
                highlighter.set_language("rust").unwrap();
                let mut code = rust_code.to_string();
                highlighter.highlight(&code).unwrap();
                (highlighter, code)
            },
            |(highlighter, mut code)| {
                code.insert_str(code.len() - 2, "println!(\"Extra line\");\n    ");
                highlighter.highlight(&code).unwrap();
            },
        )
    });

    group.finish();
}

/// Benchmarks plugin operations
fn bench_plugin_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("plugin_operations");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    // Benchmark plugin loading
    group.bench_function("plugin_load", |b| {
        b.iter_with_setup(
            || PluginManager::new(),
            |manager| {
                let plugin = MockPlugin::new("test");
                black_box(manager.register_plugin(Box::new(plugin)));
            },
        )
    });

    // Benchmark plugin command execution
    group.bench_function("plugin_execute", |b| {
        b.iter_with_setup(
            || {
                let manager = PluginManager::new();
                let plugin = MockPlugin::new("test");
                manager.register_plugin(Box::new(plugin));
                manager
            },
            |manager| {
                black_box(
                    manager.execute_command(
                        "test",
                        "test_command",
                        serde_json::json!({"arg": "value"}),
                    ),
                );
            },
        )
    });

    group.finish();
}

/// Benchmarks document operations
fn bench_document_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("document_operations");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    // Benchmark document creation
    group.bench_function("document_create", |b| {
        b.iter(|| {
            let doc = Document::new("test.txt");
            black_box(doc);
        })
    });

    // Benchmark undo/redo operations
    group.bench_function("undo_redo", |b| {
        b.iter_with_setup(
            || {
                let mut doc = Document::new("test.txt");
                doc.insert(0, "Hello, World!").unwrap();
                doc
            },
            |mut doc| {
                doc.undo().unwrap();
                doc.redo().unwrap();
            },
        )
    });

    // Benchmark large document operations
    let large_text = "Hello, World!".repeat(10000);
    group.bench_function("large_document_ops", |b| {
        b.iter_with_setup(
            || {
                let mut doc = Document::new("test.txt");
                doc.insert(0, &large_text).unwrap();
                doc
            },
            |mut doc| {
                doc.insert(1000, "New text").unwrap();
                doc.delete(1000, 1008).unwrap();
            },
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_text_operations,
    bench_syntax_highlighting,
    bench_plugin_operations,
    bench_document_operations
);
criterion_main!(benches);
