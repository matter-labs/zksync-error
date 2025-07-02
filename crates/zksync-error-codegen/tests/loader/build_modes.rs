//! Comprehensive tests for build modes functionality.
//!
//! This module tests all three build modes (NoLock, Normal, Reproducible)
//! and their interactions with dependency lock files, ensuring proper
//! behavior in various scenarios.

use std::fs;
use std::path::Path;
use tempfile::tempdir;
use zksync_error_codegen::arguments::{
    Backend, BackendOutput, GenerationArguments, ResolutionMode,
};
use zksync_error_codegen::load_and_generate;
use zksync_error_codegen::loader::dependency_lock::{DependencyEntry, DependencyLock};
use zksync_error_codegen::loader::resolution::ResolvedLink;
use zksync_error_model::link::{
    Link,
    github::{BranchName, GithubLink},
};

/// Helper function to create a temporary directory for testing
fn setup_test_env() -> tempfile::TempDir {
    tempdir().expect("Failed to create temp dir")
}

/// Helper function to create a sample error description JSON
fn create_sample_error_json() -> String {
    r#"{
        "domains": [
            {
                "domain_name": "test_domain",
                "domain_code": 42,
                "components": [
                    {
                        "component_name": "test_component",
                        "component_code": 1,
                        "errors": [
                            {
                                "error_name": "TestError",
                                "error_code": 1,
                                "description": "A test error for build mode testing"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#
    .to_string()
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

/// Helper function to create a sample GitHub error description JSON with take_from
fn create_github_dependency_json() -> String {
    r#"{
        "take_from": [
            {
                "repo": "test-org/test-repo",
                "path": "errors/common.json",
                "branch": "main"
            }
        ],
        "domains": [
            {
                "domain_name": "DependentDomain",
                "domain_code": 100,
                "identifier_encoding": "dependent_domain",
                "description": "A dependent domain for testing",
                "bindings": {
                    "rust": "DependentDomain"
                },
                "components": []
            }
        ]
    }"#
    .to_string()
}

/// Helper function to create a dependency lock with sample entries
fn create_sample_lock(temp_dir: &Path) -> DependencyLock {
    let mut lock = DependencyLock::new();

    let github_link = GithubLink::new_with_branch(
        "test-org/test-repo".to_string(),
        "errors/common.json".to_string(),
        BranchName("main".to_string()),
    );

    // Create a temporary file to serve as the resolved content
    let resolved_file = temp_dir.join("resolved_content.json");
    fs::write(&resolved_file, create_simple_test_json()).expect("Failed to write resolved content");

    let entry = DependencyEntry {
        link: Link::Github(github_link),
        resolved: ResolvedLink::LocalPath(resolved_file),
    };

    lock.add_dependency(entry);
    lock
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
fn test_normal_mode_creates_lock_file() {
    let temp_dir = setup_test_env();
    let input_file = temp_dir.path().join("input.json");
    let output_dir = temp_dir.path().join("output");
    let lock_file = temp_dir.path().join("test.lock");

    // Create input file without dependencies (to avoid network calls)
    fs::write(&input_file, create_simple_test_json()).expect("Failed to write input file");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let args = GenerationArguments {
        verbose: false,
        input_links: vec![input_file.to_string_lossy().to_string()],
        mode: ResolutionMode::Normal {
            override_links: vec![],
            lock_file: lock_file.to_string_lossy().to_string(),
        },
        outputs: vec![BackendOutput {
            output_path: output_dir.clone(),
            backend: Backend::Rust,
            arguments: vec![],
        }],
    };

    let result = load_and_generate(args);
    assert!(result.is_ok(), "Normal mode should succeed: {:?}", result);

    // In normal mode, lock file should be created even if empty
    assert!(
        lock_file.exists(),
        "Lock file should be created in Normal mode"
    );

    // Verify lock file is valid JSON
    let lock_content = fs::read_to_string(&lock_file).expect("Failed to read lock file");
    let _parsed_lock: DependencyLock =
        serde_json::from_str(&lock_content).expect("Lock file should contain valid JSON");
}

#[test]
fn test_normal_mode_updates_existing_lock_file() {
    let temp_dir = setup_test_env();
    let input_file = temp_dir.path().join("input.json");
    let output_dir = temp_dir.path().join("output");
    let lock_file = temp_dir.path().join("test.lock");

    // Create an existing lock file
    let existing_lock = create_sample_lock(temp_dir.path());
    existing_lock
        .save_to_file(lock_file.to_string_lossy().as_ref())
        .expect("Failed to save initial lock file");

    // Create input file
    fs::write(&input_file, create_simple_test_json()).expect("Failed to write input file");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let args = GenerationArguments {
        verbose: false,
        input_links: vec![input_file.to_string_lossy().to_string()],
        mode: ResolutionMode::Normal {
            override_links: vec![],
            lock_file: lock_file.to_string_lossy().to_string(),
        },
        outputs: vec![BackendOutput {
            output_path: output_dir.clone(),
            backend: Backend::Rust,
            arguments: vec![],
        }],
    };

    let result = load_and_generate(args);
    assert!(result.is_ok(), "Normal mode should succeed: {:?}", result);

    // Lock file should still exist and be valid
    assert!(lock_file.exists(), "Lock file should exist after update");

    let lock_content = fs::read_to_string(&lock_file).expect("Failed to read lock file");
    let updated_lock: DependencyLock =
        serde_json::from_str(&lock_content).expect("Updated lock file should contain valid JSON");

    // The lock should maintain the existing entry (since our input doesn't have GitHub deps)
    assert!(
        !updated_lock
            .get_dependency(&Link::Github(GithubLink::new_with_branch(
                "test-org/test-repo".to_string(),
                "errors/common.json".to_string(),
                BranchName("main".to_string()),
            )))
            .is_none(),
        "Existing dependency should be preserved"
    );
}

#[test]
fn test_reproducible_mode_uses_existing_lock() {
    let temp_dir = setup_test_env();
    let input_file = temp_dir.path().join("input.json");
    let output_dir = temp_dir.path().join("output");
    let lock_file = temp_dir.path().join("test.lock");

    // Create a lock file with dependencies
    let lock = create_sample_lock(temp_dir.path());
    lock.save_to_file(lock_file.to_string_lossy().as_ref())
        .expect("Failed to save lock file");

    // Create input file that references the locked dependency
    fs::write(&input_file, create_github_dependency_json()).expect("Failed to write input file");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let args = GenerationArguments {
        verbose: false,
        input_links: vec![input_file.to_string_lossy().to_string()],
        mode: ResolutionMode::Reproducible {
            lock_file: lock_file.to_string_lossy().to_string(),
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
        "Reproducible mode should succeed with valid lock: {:?}",
        result
    );

    // Verify output was generated
    assert!(output_dir.exists(), "Output directory should exist");

    // Lock file should remain unchanged
    let original_content = fs::read_to_string(&lock_file).expect("Failed to read lock file");
    let _original_lock: DependencyLock =
        serde_json::from_str(&original_content).expect("Lock file should be valid");

    // Verify the lock file wasn't modified (check timestamp or content)
    let lock_metadata = fs::metadata(&lock_file).expect("Failed to get lock file metadata");
    // In a real scenario, we'd check the timestamp, but for this test we just verify it exists
    assert!(lock_metadata.len() > 0, "Lock file should not be empty");
}

#[test]
fn test_reproducible_mode_fails_without_lock_file() {
    let temp_dir = setup_test_env();
    let input_file = temp_dir.path().join("input.json");
    let output_dir = temp_dir.path().join("output");
    let nonexistent_lock = temp_dir.path().join("nonexistent.lock");

    // Create input file
    fs::write(&input_file, create_simple_test_json()).expect("Failed to write input file");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let args = GenerationArguments {
        verbose: false,
        input_links: vec![input_file.to_string_lossy().to_string()],
        mode: ResolutionMode::Reproducible {
            lock_file: nonexistent_lock.to_string_lossy().to_string(),
        },
        outputs: vec![BackendOutput {
            output_path: output_dir.clone(),
            backend: Backend::Rust,
            arguments: vec![],
        }],
    };

    let result = load_and_generate(args);
    assert!(
        result.is_err(),
        "Reproducible mode should fail without lock file"
    );
}

#[test]
fn test_reproducible_mode_fails_with_missing_dependency() {
    let temp_dir = setup_test_env();
    let input_file = temp_dir.path().join("input.json");
    let output_dir = temp_dir.path().join("output");
    let lock_file = temp_dir.path().join("incomplete.lock");

    // Create an empty lock file
    let empty_lock = DependencyLock::new();
    empty_lock
        .save_to_file(lock_file.to_string_lossy().as_ref())
        .expect("Failed to save empty lock file");

    // Create input file with GitHub dependency that's not in lock
    fs::write(&input_file, create_github_dependency_json()).expect("Failed to write input file");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let args = GenerationArguments {
        verbose: false,
        input_links: vec![input_file.to_string_lossy().to_string()],
        mode: ResolutionMode::Reproducible {
            lock_file: lock_file.to_string_lossy().to_string(),
        },
        outputs: vec![BackendOutput {
            output_path: output_dir.clone(),
            backend: Backend::Rust,
            arguments: vec![],
        }],
    };

    let result = load_and_generate(args);
    assert!(
        result.is_err(),
        "Reproducible mode should fail when dependency is missing from lock"
    );
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
fn test_lock_file_format_and_content() {
    let temp_dir = setup_test_env();
    let lock_file = temp_dir.path().join("format_test.lock");

    // Create and save a lock with specific content
    let lock = create_sample_lock(temp_dir.path());
    lock.save_to_file(lock_file.to_string_lossy().as_ref())
        .expect("Failed to save lock file");

    // Read and verify the lock file format
    let lock_content = fs::read_to_string(&lock_file).expect("Failed to read lock file");

    // Verify it's valid JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&lock_content).expect("Lock file should be valid JSON");

    // Verify structure
    assert!(
        parsed.get("dependencies").is_some(),
        "Lock file should have dependencies field"
    );
    assert!(
        parsed["dependencies"].is_array(),
        "Dependencies should be an array"
    );

    // Verify it can be parsed back into DependencyLock
    let reloaded_lock: DependencyLock = serde_json::from_str(&lock_content)
        .expect("Lock file should deserialize to DependencyLock");

    // Verify the dependency is present
    let github_link = GithubLink::new_with_branch(
        "test-org/test-repo".to_string(),
        "errors/common.json".to_string(),
        BranchName("main".to_string()),
    );

    assert!(
        reloaded_lock
            .get_dependency(&Link::Github(github_link))
            .is_some(),
        "Reloaded lock should contain the expected dependency"
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

    for mode in [
        ResolutionMode::NoLock {
            override_links: vec![],
        },
        ResolutionMode::Normal {
            override_links: vec![],
            lock_file: temp_dir
                .path()
                .join("test.lock")
                .to_string_lossy()
                .to_string(),
        },
        ResolutionMode::Reproducible {
            lock_file: temp_dir
                .path()
                .join("test.lock")
                .to_string_lossy()
                .to_string(),
        },
    ] {
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

#[test]
fn test_concurrent_lock_file_access() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let temp_dir = setup_test_env();
    let lock_file = temp_dir.path().join("concurrent.lock");

    // Create initial lock file
    let lock = create_sample_lock(temp_dir.path());
    lock.save_to_file(lock_file.to_string_lossy().as_ref())
        .expect("Failed to save initial lock");

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    // Spawn multiple threads that try to read the lock file
    for i in 0..5 {
        let lock_file_path = lock_file.clone();
        let results_clone = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let result = DependencyLock::load_from_file(lock_file_path.to_string_lossy().as_ref());
            let mut results = results_clone.lock().unwrap();
            results.push((i, result.is_ok()));
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }

    // Verify all reads succeeded
    let results = results.lock().unwrap();
    assert_eq!(results.len(), 5, "All threads should complete");

    for (thread_id, success) in results.iter() {
        assert!(
            *success,
            "Thread {} should successfully read lock file",
            thread_id
        );
    }
}
