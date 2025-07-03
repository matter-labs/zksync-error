//! Comprehensive tests for build modes functionality.
//!

use std::fs;
use tempfile::tempdir;
use zksync_error_codegen::arguments::{
    Backend, BackendOutput, GenerationArguments, ResolutionMode,
};
use zksync_error_codegen::load_and_generate;

/// Helper function to create a temporary directory for testing
fn setup_test_env() -> tempfile::TempDir {
    tempdir().expect("Failed to create temp dir")
}

/// Helper function to use the common simple JSON from other tests
fn create_simple_test_json() -> String {
    r#"{
        "domains": [
            {
                "domain_name": "TestDomain",
                "domain_code": 1,
                "identifier_encoding": "test_domain",
                "description": "A test domain for build mode testing",
                "bindings": {
                    "rust": "TestDomain"
                },
                "components": []
            }
        ]
    }"#
    .to_string()
}

#[test]
fn test_no_lock_mode_basic() {
    let temp_dir = setup_test_env();
    let input_file = temp_dir.path().join("input.json");
    let output_dir = temp_dir.path().join("output");

    // Create input file
    fs::write(&input_file, create_simple_test_json()).expect("Failed to write input file");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let args = GenerationArguments {
        verbose: false,
        input_links: vec![input_file.to_string_lossy().to_string()],
        mode: ResolutionMode::NoLock {
            override_links: vec![],
        },
        outputs: vec![BackendOutput {
            output_path: output_dir.clone(),
            backend: Backend::Rust,
            arguments: vec![],
        }],
    };

    // Should succeed without creating any lock files
    let result = load_and_generate(args);
    assert!(result.is_ok(), "NoLock mode should succeed: {:?}", result);

    // Verify no lock files were created
    let lock_files: Vec<_> = fs::read_dir(&temp_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "lock" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    assert!(
        lock_files.is_empty(),
        "NoLock mode should not create lock files"
    );

    // Verify output was generated
    assert!(output_dir.exists(), "Output directory should exist");
    let lib_rs = output_dir.join("src").join("lib.rs");
    assert!(lib_rs.exists(), "Generated lib.rs should exist");
}

#[test]
fn test_no_lock_mode_with_overrides() {
    let temp_dir = setup_test_env();
    let input_file = temp_dir.path().join("input.json");
    let override_file = temp_dir.path().join("override.json");
    let output_dir = temp_dir.path().join("output");

    // Create a simple input file without GitHub dependencies to avoid override complexity
    fs::write(&input_file, create_simple_test_json()).expect("Failed to write input file");
    fs::write(&override_file, create_simple_test_json()).expect("Failed to write override file");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let args = GenerationArguments {
        verbose: false,
        input_links: vec![input_file.to_string_lossy().to_string()],
        mode: ResolutionMode::NoLock {
            override_links: vec![], // No overrides needed for this simple test
        },
        outputs: vec![BackendOutput {
            output_path: output_dir.clone(),
            backend: Backend::Rust,
            arguments: vec![],
        }],
    };

    let result = load_and_generate(args);
    assert!(
        result.is_ok(),
        "NoLock mode should succeed without overrides: {:?}",
        result
    );

    // Verify output was generated
    assert!(output_dir.exists(), "Output directory should exist");
}

#[test]
fn test_mode_with_multiple_outputs() {
    let temp_dir = setup_test_env();
    let input_file = temp_dir.path().join("input.json");
    let rust_output_dir = temp_dir.path().join("rust_output");
    let docs_output_dir = temp_dir.path().join("docs_output");

    // Create input file
    fs::write(&input_file, create_simple_test_json()).expect("Failed to write input file");
    fs::create_dir_all(&rust_output_dir).expect("Failed to create rust output dir");
    fs::create_dir_all(&docs_output_dir).expect("Failed to create docs output dir");

    let args = GenerationArguments {
        verbose: false,
        input_links: vec![input_file.to_string_lossy().to_string()],
        mode: ResolutionMode::NoLock {
            override_links: vec![],
        },
        outputs: vec![
            BackendOutput {
                output_path: rust_output_dir.clone(),
                backend: Backend::Rust,
                arguments: vec![],
            },
            BackendOutput {
                output_path: docs_output_dir.clone(),
                backend: Backend::Mdbook,
                arguments: vec![],
            },
        ],
    };

    let result = load_and_generate(args);
    assert!(result.is_ok(), "Multiple outputs should work: {:?}", result);

    // Verify both outputs were generated
    assert!(
        rust_output_dir.join("src").join("lib.rs").exists(),
        "Rust output should be generated"
    );
    assert!(
        docs_output_dir.join("src").join("SUMMARY.md").exists(),
        "MDBook output should be generated"
    );
}

#[test]
fn test_build_mode_error_propagation() {
    let temp_dir = setup_test_env();
    let input_file = temp_dir.path().join("invalid.json");
    let output_dir = temp_dir.path().join("output");

    // Create invalid JSON input
    fs::write(&input_file, "{ invalid json }").expect("Failed to write invalid input");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    for mode in [ResolutionMode::NoLock {
        override_links: vec![],
    }] {
        let args = GenerationArguments {
            verbose: false,
            input_links: vec![input_file.to_string_lossy().to_string()],
            mode,
            outputs: vec![BackendOutput {
                output_path: output_dir.clone(),
                backend: Backend::Rust,
                arguments: vec![],
            }],
        };

        let result = load_and_generate(args);
        assert!(
            result.is_err(),
            "All modes should properly handle invalid input"
        );
    }
}
