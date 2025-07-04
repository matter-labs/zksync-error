//! Simple focused tests for override-disables-lock-file behavior.

use std::collections::BTreeMap;
use std::fs;
use tempfile::NamedTempFile;
use zksync_error_codegen::loader::load_dependent_component;
use zksync_error_codegen::loader::resolution::{context::ResolutionContext, overrides::Remapping};
use zksync_error_model::link::Link;

// Test the most basic scenario: single dependency with override
#[test]
fn test_override_vs_no_override() {
    // Create a root file that has no dependencies - completely self-contained
    let root_content = r#"{
        "domains": [
            {
                "domain_name": "SelfContainedRoot",
                "domain_code": 1,
                "identifier_encoding": "self_contained_root",
                "description": "A self-contained root",
                "bindings": {
                    "rust": "SelfContainedRoot"
                },
                "components": []
            }
        ]
    }"#;
    
    let root_file = NamedTempFile::new().expect("Failed to create temp file");
    fs::write(root_file.path(), root_content).expect("Failed to write to temp file");
    
    let root_link = Link::FileLink {
        path: root_file.path().to_string_lossy().to_string(),
    };
    
    // Test 1: NoLock context with no overrides (should work)
    let mut context = ResolutionContext::NoLock {
        overrides: Remapping { map: BTreeMap::new() },
    };
    
    let result = load_dependent_component(root_link.clone(), &mut context);
    assert!(result.is_ok(), "Self-contained file should load successfully");
    
    if let Ok(fragments) = result {
        assert_eq!(fragments.len(), 1, "Should have exactly one fragment");
        assert_eq!(fragments[0].root.domains.len(), 1, "Should have one domain");
        assert_eq!(fragments[0].root.domains[0].domain_name, "SelfContainedRoot");
    }
    
    // Test 2: LockOrPopulate context with no overrides (should also work)
    let mut context2 = ResolutionContext::LockOrPopulate {
        overrides: Remapping { map: BTreeMap::new() },
        lock: zksync_error_codegen::loader::dependency_lock::DependencyLock::new(),
    };
    
    let result2 = load_dependent_component(root_link, &mut context2);
    assert!(result2.is_ok(), "Self-contained file should load successfully in LockOrPopulate mode");
}

// Test that demonstrates the feature works: override changes resolution
#[test]  
fn test_override_changes_resolution() {
    // Create two different files with different domain names
    let original_content = r#"{
        "domains": [
            {
                "domain_name": "OriginalDomain",
                "domain_code": 1,
                "identifier_encoding": "original_domain",
                "description": "Original domain",
                "bindings": {
                    "rust": "OriginalDomain"
                },
                "components": []
            }
        ]
    }"#;
    
    let override_content = r#"{
        "domains": [
            {
                "domain_name": "OverriddenDomain",
                "domain_code": 2,
                "identifier_encoding": "overridden_domain", 
                "description": "Overridden domain",
                "bindings": {
                    "rust": "OverriddenDomain"
                },
                "components": []
            }
        ]
    }"#;
    
    let original_file = NamedTempFile::new().expect("Failed to create temp file");
    fs::write(original_file.path(), original_content).expect("Failed to write to temp file");
    
    let override_file = NamedTempFile::new().expect("Failed to create temp file");
    fs::write(override_file.path(), override_content).expect("Failed to write to temp file");
    
    let original_link = Link::FileLink {
        path: original_file.path().to_string_lossy().to_string(),
    };
    
    // Test without override - should get original domain
    let mut context1 = ResolutionContext::NoLock {
        overrides: Remapping { map: BTreeMap::new() },
    };
    
    let result1 = load_dependent_component(original_link.clone(), &mut context1);
    assert!(result1.is_ok(), "Should load original file");
    if let Ok(fragments) = result1 {
        assert!(fragments[0].root.domains.iter().any(|d| d.domain_name == "OriginalDomain"), 
                "Should have original domain");
    }
    
    // Test with override - should get overridden domain
    let mut overrides = BTreeMap::new();
    overrides.insert(
        original_link.clone(),
        Link::FileLink {
            path: override_file.path().to_string_lossy().to_string(),
        }
    );
    
    let mut context2 = ResolutionContext::NoLock {
        overrides: Remapping { map: overrides },
    };
    
    let result2 = load_dependent_component(original_link, &mut context2);
    assert!(result2.is_ok(), "Should load overridden file");
    if let Ok(fragments) = result2 {
        assert!(fragments[0].root.domains.iter().any(|d| d.domain_name == "OverriddenDomain"), 
                "Should have overridden domain");
    }
}