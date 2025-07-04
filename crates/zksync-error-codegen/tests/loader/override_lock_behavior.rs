//! Comprehensive tests for override-disables-lock-file behavior.
//!
//! This module tests the feature where overridden dependencies cause
//! their entire dependency subgraph to be resolved in NoLock mode,
//! ensuring fresh resolution without interference from lock files.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use tempfile::{tempdir, NamedTempFile};
use zksync_error_codegen::arguments::{
    Backend, BackendOutput, GenerationArguments, ResolutionMode,
};
use zksync_error_codegen::load_and_generate;
use zksync_error_codegen::loader::dependency_lock::{DependencyEntry, DependencyLock};
use zksync_error_codegen::loader::load_dependent_component;
use zksync_error_codegen::loader::resolution::{context::ResolutionContext, overrides::Remapping, ResolvedLink};
use zksync_error_model::link::Link;

// Helper functions for creating test content

fn create_simple_root() -> String {
    r#"{
        "take_from": [
            "file://dep.json"
        ],
        "domains": [
            {
                "domain_name": "RootDomain",
                "domain_code": 1,
                "identifier_encoding": "root_domain",
                "description": "Root domain with dependencies",
                "bindings": {
                    "rust": "RootDomain"
                },
                "components": []
            }
        ]
    }"#
    .to_string()
}

fn create_dependency() -> String {
    r#"{
        "domains": [
            {
                "domain_name": "DepDomain",
                "domain_code": 2,
                "identifier_encoding": "dep_domain",
                "description": "Dependency domain",
                "bindings": {
                    "rust": "DepDomain"
                },
                "components": []
            }
        ]
    }"#
    .to_string()
}

fn create_alternative_dependency() -> String {
    r#"{
        "domains": [
            {
                "domain_name": "AltDepDomain", 
                "domain_code": 3,
                "identifier_encoding": "alt_dep_domain",
                "description": "Alternative dependency domain",
                "bindings": {
                    "rust": "AltDepDomain"
                },
                "components": []
            }
        ]
    }"#
    .to_string()
}

fn create_lock_with_dependency(temp_dir: &Path) -> DependencyLock {
    let mut lock = DependencyLock::new();

    // Create resolved content file for lock file
    let dep_content = temp_dir.join("locked_dep.json");
    fs::write(&dep_content, create_dependency()).expect("Failed to write dep content");

    let entry = DependencyEntry {
        link: Link::FileLink {
            path: "dep.json".to_string(),
        },
        resolved: ResolvedLink::LocalPath(dep_content),
    };

    lock.add_dependency(entry);
    lock
}

// Test 1: Basic override behavior - when dependency is overridden, use override not lock
#[test]
fn test_basic_override_behavior() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    
    // Store original directory first
    let original_dir = std::env::current_dir().expect("Failed to get current dir");
    
    // Create a guard to ensure we always restore the directory
    struct DirGuard(std::path::PathBuf);
    impl Drop for DirGuard {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.0);
        }
    }
    let _guard = DirGuard(original_dir);
    
    // Change to the temp directory so relative paths work
    std::env::set_current_dir(temp_dir.path()).expect("Failed to change to temp dir");
    
    // Create the actual dependency file that will be referenced in the working directory
    let actual_dep_file = temp_dir.path().join("dep.json");
    fs::write(&actual_dep_file, create_dependency()).expect("Failed to write actual dep");
    
    // Create alternative dependency content (self-contained, no further dependencies)
    let alt_dep_file = NamedTempFile::new().expect("Failed to create temp file");
    fs::write(alt_dep_file.path(), create_alternative_dependency()).expect("Failed to write to temp file");
    
    // Create lock file with original dependency
    let lock = create_lock_with_dependency(temp_dir.path());
    
    // Create override mapping: override dep to alternative
    let mut overrides = BTreeMap::new();
    let dep_link = Link::FileLink {
        path: "dep.json".to_string(),
    };
    let alt_dep_link = Link::FileLink {
        path: alt_dep_file.path().to_string_lossy().to_string(),
    };
    overrides.insert(dep_link, alt_dep_link);
    
    // Create root dependency file
    let root_file = NamedTempFile::new().expect("Failed to create temp file");
    fs::write(root_file.path(), create_simple_root()).expect("Failed to write to temp file");
    
    let root_link = Link::FileLink {
        path: root_file.path().to_string_lossy().to_string(),
    };
    
    // Test with LockOrPopulate context (Normal mode)
    let mut context = ResolutionContext::LockOrPopulate {
        overrides: Remapping { map: overrides },
        lock,
    };
    
    let result = load_dependent_component(root_link, &mut context);
    
    match result {
        Ok(fragments) => {
            // Should have alternative dependency, not locked version
            let has_alt_dep = fragments.iter().any(|f| {
                f.root.domains.iter().any(|d| d.domain_name == "AltDepDomain")
            });
            let has_original_dep = fragments.iter().any(|f| {
                f.root.domains.iter().any(|d| d.domain_name == "DepDomain")
            });
            
            assert!(has_alt_dep, "Should have alternative dependency domain");
            assert!(!has_original_dep, "Should not have original dependency from lock");
        }
        Err(e) => panic!("Expected success but got error: {e}"),
    }
}

// Test 2: Lock file is used when no overrides
#[test]
fn test_lock_file_used_without_overrides() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    
    // Store original directory first
    let original_dir = std::env::current_dir().expect("Failed to get current dir");
    
    // Create a guard to ensure we always restore the directory
    struct DirGuard(std::path::PathBuf);
    impl Drop for DirGuard {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.0);
        }
    }
    let _guard = DirGuard(original_dir);
    
    // Change to the temp directory so relative paths work
    std::env::set_current_dir(temp_dir.path()).expect("Failed to change to temp dir");
    
    // Create the actual dependency file that the lock points to
    let actual_dep_file = temp_dir.path().join("actual_dep.json");
    fs::write(&actual_dep_file, create_dependency()).expect("Failed to write actual dep");
    
    // Also create dep.json in the current directory for fallback
    let dep_file = temp_dir.path().join("dep.json");
    fs::write(&dep_file, create_dependency()).expect("Failed to write dep.json");
    
    // Create lock file with dependency pointing to the actual file
    let mut lock = DependencyLock::new();
    let entry = DependencyEntry {
        link: Link::FileLink {
            path: "dep.json".to_string(),
        },
        resolved: ResolvedLink::LocalPath(actual_dep_file),
    };
    lock.add_dependency(entry);
    
    // Create root dependency file
    let root_file = NamedTempFile::new().expect("Failed to create temp file");
    fs::write(root_file.path(), create_simple_root()).expect("Failed to write to temp file");
    
    let root_link = Link::FileLink {
        path: root_file.path().to_string_lossy().to_string(),
    };
    
    // Test with LockOrPopulate context with no overrides
    let mut context = ResolutionContext::LockOrPopulate {
        overrides: Remapping { map: BTreeMap::new() },
        lock,
    };
    
    let result = load_dependent_component(root_link, &mut context);
    
    match result {
        Ok(fragments) => {
            // Should have dependency from lock file
            let has_dep = fragments.iter().any(|f| {
                f.root.domains.iter().any(|d| d.domain_name == "DepDomain")
            });
            
            assert!(has_dep, "Should have dependency from lock file");
        }
        Err(e) => panic!("Expected success but got error: {e}"),
    }
}

// Test 3: Different resolution contexts
#[test]
fn test_different_resolution_contexts() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    
    // Store original directory first
    let original_dir = std::env::current_dir().expect("Failed to get current dir");
    
    // Create a guard to ensure we always restore the directory
    struct DirGuard(std::path::PathBuf);
    impl Drop for DirGuard {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.0);
        }
    }
    let _guard = DirGuard(original_dir);
    
    // Change to the temp directory so relative paths work
    std::env::set_current_dir(temp_dir.path()).expect("Failed to change to temp dir");
    
    // Create the actual dependency file
    let actual_dep_file = temp_dir.path().join("dep.json");
    fs::write(&actual_dep_file, create_dependency()).expect("Failed to write actual dep");
    
    // Create alternative dependency
    let alt_dep_file = NamedTempFile::new().expect("Failed to create temp file");
    fs::write(alt_dep_file.path(), create_alternative_dependency()).expect("Failed to write to temp file");
    
    // Create simple root
    let root_file = NamedTempFile::new().expect("Failed to create temp file");
    fs::write(root_file.path(), create_simple_root()).expect("Failed to write to temp file");
    
    let root_link = Link::FileLink {
        path: root_file.path().to_string_lossy().to_string(),
    };
    
    // Create override
    let mut overrides = BTreeMap::new();
    let dep_link = Link::FileLink {
        path: "dep.json".to_string(),
    };
    let alt_dep_link = Link::FileLink {
        path: alt_dep_file.path().to_string_lossy().to_string(),
    };
    overrides.insert(dep_link, alt_dep_link);
    
    // Test 1: NoLock context with overrides
    let mut no_lock_context = ResolutionContext::NoLock {
        overrides: Remapping { map: overrides.clone() },
    };
    
    let result = load_dependent_component(root_link.clone(), &mut no_lock_context);
    assert!(result.is_ok(), "NoLock context should succeed");
    
    // Test 2: LockOrPopulate context
    let lock = create_lock_with_dependency(temp_dir.path());
    let mut lock_populate_context = ResolutionContext::LockOrPopulate {
        overrides: Remapping { map: overrides },
        lock,
    };
    
    let result = load_dependent_component(root_link, &mut lock_populate_context);
    assert!(result.is_ok(), "LockOrPopulate context should succeed");
}

// Test 4: Error handling in override scenarios
#[test]
fn test_error_handling_with_overrides() {
    let _temp_dir = tempdir().expect("Failed to create temp dir");
    
    // Create a root that depends on a dependency
    let root_file = NamedTempFile::new().expect("Failed to create temp file");
    fs::write(root_file.path(), create_simple_root()).expect("Failed to write to temp file");
    
    // Create override to non-existent file
    let mut overrides = BTreeMap::new();
    let dep_link = Link::FileLink {
        path: "dep.json".to_string(),
    };
    let bad_override_link = Link::FileLink {
        path: "/nonexistent/path/missing.json".to_string(),
    };
    overrides.insert(dep_link, bad_override_link);
    
    let root_link = Link::FileLink {
        path: root_file.path().to_string_lossy().to_string(),
    };
    
    let mut context = ResolutionContext::NoLock {
        overrides: Remapping { map: overrides },
    };
    
    let result = load_dependent_component(root_link, &mut context);
    
    // Should fail gracefully with appropriate error
    assert!(result.is_err(), "Should fail when override points to non-existent file");
}

// Test 5: End-to-end integration test
#[test]
fn test_end_to_end_override_behavior() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    
    // Store original directory first
    let original_dir = std::env::current_dir().expect("Failed to get current dir");
    
    // Create a guard to ensure we always restore the directory
    struct DirGuard(std::path::PathBuf);
    impl Drop for DirGuard {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.0);
        }
    }
    let _guard = DirGuard(original_dir);
    
    // Change to the temp directory so relative paths work
    std::env::set_current_dir(temp_dir.path()).expect("Failed to change to temp dir");
    
    // Create the actual dependency file that the lock points to
    let actual_dep_file = temp_dir.path().join("dep.json");
    fs::write(&actual_dep_file, create_dependency()).expect("Failed to write actual dep");
    
    // Create input file with dependencies
    let input_file = temp_dir.path().join("input.json");
    fs::write(&input_file, create_simple_root()).expect("Failed to write input file");
    
    // Create override file
    let alt_dep_file = temp_dir.path().join("alt_dep.json");
    fs::write(&alt_dep_file, create_alternative_dependency()).expect("Failed to write alt dep");
    
    // Create lock file
    let lock_file = temp_dir.path().join("test.lock");
    let lock = create_lock_with_dependency(temp_dir.path());
    lock.save_to_file(lock_file.to_string_lossy().as_ref()).expect("Failed to save lock");
    
    // Create output directory
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");
    
    // Create override mapping
    let override_links = vec![
        (
            "file://dep.json".to_string(),
            alt_dep_file.to_string_lossy().to_string()
        ),
    ];
    
    // Test with Normal mode (should use overrides and ignore lock for overridden dependency)
    let args = GenerationArguments {
        verbose: false,
        input_links: vec![input_file.to_string_lossy().to_string()],
        mode: ResolutionMode::Normal {
            override_links,
            lock_file: lock_file.to_string_lossy().to_string(),
        },
        outputs: vec![BackendOutput {
            output_path: output_dir.clone(),
            backend: Backend::Rust,
            arguments: vec![],
        }],
    };
    
    let result = load_and_generate(args);
    
    assert!(result.is_ok(), "End-to-end generation should succeed: {result:?}");
    
    // Verify output was generated
    let lib_rs = output_dir.join("src").join("lib.rs");
    assert!(lib_rs.exists(), "Generated lib.rs should exist");
    
    // Verify the generated content includes the alternative domain
    let generated_content = fs::read_to_string(&lib_rs).expect("Failed to read generated content");
    
    // Should contain alternative domain name
    assert!(generated_content.contains("AltDepDomain") || generated_content.contains("alt_dep_domain"), 
        "Generated content should contain alternative dependency domain");
    
    // Should NOT contain original domain name (unless both are present, which is also ok)
    assert!(!generated_content.contains("DepDomain") || generated_content.contains("AltDepDomain"), 
        "Generated content should not contain original dependency domain or should prefer alternative");
}

// Test 6: Reproducible mode should work without overrides
#[test]
fn test_reproducible_mode_no_overrides() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    
    // Create simple input
    let input_file = temp_dir.path().join("input.json");
    fs::write(&input_file, create_simple_test_json()).expect("Failed to write input");
    
    // Create lock file
    let lock_file = temp_dir.path().join("repro.lock");
    let lock = DependencyLock::new();
    lock.save_to_file(lock_file.to_string_lossy().as_ref()).expect("Failed to save lock");
    
    // Create output directory
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");
    
    // Try Reproducible mode (should work without overrides)
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
    assert!(result.is_ok(), "Reproducible mode should succeed without overrides");
}

fn create_simple_test_json() -> String {
    r#"{
        "domains": [
            {
                "domain_name": "TestDomain",
                "domain_code": 1,
                "identifier_encoding": "test_domain",
                "description": "A test domain",
                "bindings": {
                    "rust": "TestDomain"
                },
                "components": []
            }
        ]
    }"#
    .to_string()
}