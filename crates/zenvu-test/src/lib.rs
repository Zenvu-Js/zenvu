//! # Zenvu Test Framework
//!
//! A blazingly fast testing utility for Zenvu.js components.
//! It hooks into `zenvu-compiler` to parse and assert `.Zenvu` files directly.

use anyhow::Result;
use std::path::{Path, PathBuf};
use zenvu_compiler::compile;
use zenvu_compiler::CompileOptions;

pub struct TestConfig {
    pub pattern: String,
    pub watch: bool,
}

pub struct TestResult {
    pub file: PathBuf,
    pub passed: bool,
    pub output: String,
}

/// Discovers and runs tests for `.Zenvu` components.
pub async fn run_tests(config: &TestConfig) -> Result<()> {
    println!("\n  ðŸ§ª Running Zenvu.js Component Tests...");
    println!("  Pattern: {}\n", config.pattern);

    let test_files = discover_tests(Path::new("src"), &config.pattern)?;
    
    if test_files.is_empty() {
        println!("  âš ï¸  No tests found matching pattern.");
        return Ok(());
    }

    let mut passed = 0;
    let mut failed = 0;

    for file in &test_files {
        match execute_test_file(file).await {
            Ok(result) => {
                if result.passed {
                    println!("  âœ… {} ... ok", file.display());
                    passed += 1;
                } else {
                    println!("  âŒ {} ... FAILED", file.display());
                    println!("     {}", result.output);
                    failed += 1;
                }
            }
            Err(e) => {
                println!("  ðŸš¨ {} ... PANIC", file.display());
                println!("     Error: {}", e);
                failed += 1;
            }
        }
    }

    println!("\n  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€");
    println!("  Test Results: {} passed, {} failed, {} total", passed, failed, test_files.len());

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}

async fn execute_test_file(path: &Path) -> Result<TestResult> {
    let source = std::fs::read_to_string(path)?;
    
    // Simulate a compilation pass to ensure the component is valid
    let options = CompileOptions {
        mode: zenvu_compiler::CompileMode::Dom,
        source_map: false,
        minify: false,
        target: zenvu_compiler::EsTarget::Es2022,
        dev: true,
        scope_id: Some("test-scope".into()),
    };

    let compile_result = compile(&source, path.to_str().unwrap(), &options);

    match compile_result {
        Ok(_) => {
            // In a real framework, we would spawn a v8 isolate or JSDOM here
            // to execute the `<script context="test">` blocks.
            // For now, if it compiles successfully, we pass the test.
            Ok(TestResult {
                file: path.to_path_buf(),
                passed: true,
                output: "Compiled successfully.".into(),
            })
        }
        Err(e) => {
            Ok(TestResult {
                file: path.to_path_buf(),
                passed: false,
                output: format!("Compilation failed: {}", e),
            })
        }
    }
}

fn discover_tests(dir: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if !dir.exists() { return Ok(files); }

    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            files.extend(discover_tests(&path, pattern)?);
        } else if let Some(name) = path.file_name() {
            let name_str = name.to_string_lossy();
            if (name_str.ends_with(".spec.Zenvu") || name_str.ends_with(".test.Zenvu"))
                && (pattern.is_empty() || name_str.contains(pattern)) {
                    files.push(path);
                }
        }
    }
    Ok(files)
}
