//! Integration tests for build mode complex scenarios.
//!
//! This module tests advanced build mode scenarios including mode transitions,
//! complex dependency graphs, and real-world usage patterns.

use std::fs;
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


#[test]
fn test_normal_to_reproducible_workflow() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let input_file = temp_dir.path().join("input.json");
    let output_dir = temp_dir.path().join("output");
    let lock_file = temp_dir.path().join("workflow.lock");

    // Create input with GitHub dependencies (but use local files to avoid network)
    fs::write(
        &input_file,
        r#"{
        "domains": [
            {
                "domain_name": "WorkflowDomain",
                "domain_code": 300,
                "identifier_encoding": "workflow_domain",
                "description": "A workflow domain for testing",
                "bindings": {
                    "rust": "WorkflowDomain"
                },
                "components": []
            }
        ]
    }"#,
    )
    .expect("Failed to write input file");

    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    // Step 1: Run in Normal mode to create lock file
    let normal_args = GenerationArguments {
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

    let result = load_and_generate(normal_args);
    assert!(result.is_ok(), "Normal mode should succeed: {result:?}");
    assert!(lock_file.exists(), "Lock file should be created");

    // Step 2: Run in Reproducible mode using the same lock file
    let reproducible_args = GenerationArguments {
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

    let result = load_and_generate(reproducible_args);
    assert!(
        result.is_ok(),
        "Reproducible mode should succeed with existing lock: {result:?}"
    );

    // Verify outputs are consistent
    let lib_rs = output_dir.join("src").join("lib.rs");
    assert!(lib_rs.exists(), "Output should be generated consistently");
}

#[test]
fn test_lock_file_backwards_compatibility() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let lock_file = temp_dir.path().join("compat.lock");

    // Create a lock file manually with known format (using correct ReferenceType format)
    let lock_json = r#"{
        "dependencies": [
            {
                "link": {
                    "Github": {
                        "repo": "test/repo",
                        "path": "test.json",
                        "branch": "main"
                    }
                },
                "resolved": {
                    "LocalPath": "/tmp/resolved.json"
                }
            }
        ]
    }"#;

    fs::write(&lock_file, lock_json).expect("Failed to write lock file");

    // Try to load the lock file
    let result = DependencyLock::load_from_file(lock_file.to_string_lossy().as_ref());
    assert!(
        result.is_ok(),
        "Should be able to load manually created lock file: {result:?}"
    );

    let lock = result.unwrap();

    // Verify the dependency can be found
    let github_link = GithubLink::new_with_branch(
        "test/repo".to_string(),
        "test.json".to_string(),
        BranchName("main".to_string()),
    );

    let found = lock.get_dependency(&Link::Github(github_link));
    assert!(
        found.is_some(),
        "Should find the dependency in the loaded lock"
    );
}

#[test]
fn test_build_mode_with_invalid_lock_file() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let input_file = temp_dir.path().join("input.json");
    let output_dir = temp_dir.path().join("output");
    let invalid_lock = temp_dir.path().join("invalid.lock");

    // Create input file
    fs::write(
        &input_file,
        r#"{
        "domains": [
            {
                "domain_name": "SimpleDomain",
                "domain_code": 1,
                "identifier_encoding": "simple_domain",
                "description": "A simple domain for testing",
                "bindings": {
                    "rust": "SimpleDomain"
                },
                "components": []
            }
        ]
    }"#,
    )
    .expect("Failed to write input");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    // Create invalid lock file
    fs::write(&invalid_lock, "{ invalid json content }").expect("Failed to write invalid lock");

    // Test Normal mode with invalid lock file - should handle gracefully
    let normal_args = GenerationArguments {
        verbose: false,
        input_links: vec![input_file.to_string_lossy().to_string()],
        mode: ResolutionMode::Normal {
            override_links: vec![],
            lock_file: invalid_lock.to_string_lossy().to_string(),
        },
        outputs: vec![BackendOutput {
            output_path: output_dir.clone(),
            backend: Backend::Rust,
            arguments: vec![],
        }],
    };

    // Normal mode handles invalid lock gracefully by creating a new one
    let result = load_and_generate(normal_args);
    assert!(
        result.is_ok(),
        "Normal mode should handle invalid lock gracefully"
    );

    // Create a new invalid lock file for reproducible mode test
    let invalid_lock2 = temp_dir.path().join("invalid2.lock");
    fs::write(&invalid_lock2, "{ invalid json content }").expect("Failed to write invalid lock");

    // Test Reproducible mode with invalid lock file
    let repro_args = GenerationArguments {
        verbose: false,
        input_links: vec![input_file.to_string_lossy().to_string()],
        mode: ResolutionMode::Reproducible {
            lock_file: invalid_lock2.to_string_lossy().to_string(),
        },
        outputs: vec![BackendOutput {
            output_path: output_dir.clone(),
            backend: Backend::Rust,
            arguments: vec![],
        }],
    };

    let result = load_and_generate(repro_args);
    assert!(
        result.is_err(),
        "Reproducible mode should fail with invalid lock file"
    );
}

#[test]
fn test_lock_file_with_special_characters() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let special_lock = temp_dir.path().join("special-chars@#$.lock");

    // Create lock with special characters in paths and names
    let mut lock = DependencyLock::new();

    let special_content = temp_dir
        .path()
        .join("special file with spaces & chars.json");
    fs::write(
        &special_content,
        r#"{
        "domains": [
            {
                "domain_name": "SpecialDomain",
                "domain_code": 1,
                "identifier_encoding": "special_domain",
                "description": "A special domain for testing",
                "bindings": {
                    "rust": "SpecialDomain"
                },
                "components": []
            }
        ]
    }"#,
    )
    .expect("Failed to write special content");

    let entry = DependencyEntry {
        link: Link::Github(GithubLink::new_with_branch(
            "org/repo-with-dashes".to_string(),
            "path/with spaces/file.json".to_string(),
            BranchName("feature/special-branch".to_string()),
        )),
        resolved: ResolvedLink::LocalPath(special_content),
    };

    lock.add_dependency(entry);

    // Save and reload
    let result = lock.save_to_file(special_lock.to_string_lossy().as_ref());
    assert!(
        result.is_ok(),
        "Should be able to save lock with special characters: {result:?}"
    );

    let reloaded = DependencyLock::load_from_file(special_lock.to_string_lossy().as_ref());
    assert!(
        reloaded.is_ok(),
        "Should be able to load lock with special characters: {reloaded:?}"
    );

    let reloaded_lock = reloaded.unwrap();
    let query_link = GithubLink::new_with_branch(
        "org/repo-with-dashes".to_string(),
        "path/with spaces/file.json".to_string(),
        BranchName("feature/special-branch".to_string()),
    );

    assert!(
        reloaded_lock
            .get_dependency(&Link::Github(query_link))
            .is_some(),
        "Should find dependency with special characters"
    );
}

#[test]
fn test_build_mode_performance_with_large_lock() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let large_lock_file = temp_dir.path().join("large.lock");

    // Create a lock file with many entries
    let mut lock = DependencyLock::new();

    for i in 0..100 {
        let content_file = temp_dir.path().join(format!("content_{i}.json"));
        fs::write(
            &content_file,
            r#"{
            "domains": [
                {
                    "domain_name": "LargeDomain",
                    "domain_code": 1,
                    "identifier_encoding": "large_domain",
                    "description": "A large domain for testing",
                    "bindings": {
                        "rust": "LargeDomain"
                    },
                    "components": []
                }
            ]
        }"#,
        )
        .expect("Failed to write content");

        let entry = DependencyEntry {
            link: Link::Github(GithubLink::new_with_branch(
                format!("org/repo-{i}"),
                format!("path/file-{i}.json"),
                BranchName("main".to_string()),
            )),
            resolved: ResolvedLink::LocalPath(content_file),
        };

        lock.add_dependency(entry);
    }

    // Save the large lock file
    let start_time = std::time::Instant::now();
    let result = lock.save_to_file(large_lock_file.to_string_lossy().as_ref());
    let save_duration = start_time.elapsed();

    assert!(
        result.is_ok(),
        "Should be able to save large lock file: {result:?}"
    );
    assert!(
        save_duration.as_millis() < 1000,
        "Save should complete within reasonable time"
    );

    // Test loading performance
    let start_time = std::time::Instant::now();
    let reloaded = DependencyLock::load_from_file(large_lock_file.to_string_lossy().as_ref());
    let load_duration = start_time.elapsed();

    assert!(
        reloaded.is_ok(),
        "Should be able to load large lock file: {reloaded:?}"
    );
    assert!(
        load_duration.as_millis() < 1000,
        "Load should complete within reasonable time"
    );

    // Test lookup performance
    let reloaded_lock = reloaded.unwrap();
    let query_link = GithubLink::new_with_branch(
        "org/repo-50".to_string(),
        "path/file-50.json".to_string(),
        BranchName("main".to_string()),
    );

    let start_time = std::time::Instant::now();
    let found = reloaded_lock.get_dependency(&Link::Github(query_link));
    let lookup_duration = start_time.elapsed();

    assert!(found.is_some(), "Should find dependency in large lock");
    assert!(
        lookup_duration.as_micros() < 10000,
        "Lookup should be fast even in large lock"
    );
}

#[test]
fn test_mode_switching_preserves_outputs() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let input_file = temp_dir.path().join("input.json");
    let output_dir = temp_dir.path().join("output");
    let lock_file = temp_dir.path().join("switch.lock");

    // Create consistent input using the exact same format as working tests
    let input_content = r#"{
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
    }"#;

    fs::write(&input_file, input_content).expect("Failed to write input");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    // Generate with Normal mode
    let normal_result = load_and_generate(GenerationArguments {
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
    });
    assert!(
        normal_result.is_ok(),
        "Normal mode should succeed: {normal_result:?}"
    );

    let normal_output = fs::read_to_string(output_dir.join("src").join("lib.rs"))
        .expect("Should be able to read normal mode output");

    // Clear output directory
    fs::remove_dir_all(&output_dir).expect("Failed to remove output dir");
    fs::create_dir_all(&output_dir).expect("Failed to recreate output dir");

    // Generate with Reproducible mode using the same lock
    let repro_result = load_and_generate(GenerationArguments {
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
    });
    assert!(repro_result.is_ok(), "Reproducible mode should succeed");

    let repro_output = fs::read_to_string(output_dir.join("src").join("lib.rs"))
        .expect("Should be able to read reproducible mode output");

    // Outputs should be identical
    assert_eq!(
        normal_output, repro_output,
        "Output should be identical between Normal and Reproducible modes"
    );
}
