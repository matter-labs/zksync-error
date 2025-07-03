use std::collections::BTreeMap;
use std::io::Write;
use tempfile::NamedTempFile;
use zksync_error_codegen::loader::load_dependent_component;
use zksync_error_model::link::Link;
use zksync_error_model::link::github::{BranchName, GithubLink, ReferenceType};

use super::common::*;

#[test]
fn test_load_dependent_component_with_overrides() {
    // Create actual file
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    write!(temp_file, "{}", create_simple_json_content()).expect("Failed to write to temp file");

    let actual_path = temp_file.path().to_string_lossy().to_string();
    let actual_link = Link::FileLink { path: actual_path };

    // Create override mapping
    let virtual_link = Link::FileLink {
        path: "/virtual/path.json".to_string(),
    };
    let mut overrides = BTreeMap::new();
    overrides.insert(virtual_link.clone(), actual_link);

    let mut context = create_test_context_with_overrides(overrides);

    let result = load_dependent_component(virtual_link, &mut context);

    match result {
        Ok(fragments) => {
            assert_eq!(fragments.len(), 1);
            assert_eq!(fragments[0].root.domains.len(), 1);
        }
        Err(e) => panic!("Expected success with overrides but got error: {e}"),
    }
}

#[test]
fn test_load_dependent_component_github_link() {
    // Test with a GitHub link - this would normally require network access
    // so we'll use overrides to redirect to a local file
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    write!(temp_file, "{}", create_simple_json_content()).expect("Failed to write to temp file");

    let actual_link = Link::FileLink {
        path: temp_file.path().to_string_lossy().to_string(),
    };
    let github_link = Link::Github(GithubLink {
        repo: "test/test".to_string(),
        path: "test.json".to_string(),
        reference: ReferenceType::Branch {
            branch: BranchName("main".to_string()),
        },
    });

    let mut overrides = BTreeMap::new();
    overrides.insert(github_link.clone(), actual_link);
    let mut context = create_test_context_with_overrides(overrides);

    let result = load_dependent_component(github_link, &mut context);

    match result {
        Ok(fragments) => {
            assert_eq!(fragments.len(), 1);
            assert_eq!(fragments[0].root.domains.len(), 1);
        }
        Err(e) => panic!("Expected success with GitHub link override but got error: {e}"),
    }
}
