use std::io::Write;
use tempfile::NamedTempFile;
use zksync_error_codegen::loader::{error::LoadError, load_dependent_component};
use zksync_error_model::link::Link;

use super::common::*;

#[test]
fn test_load_dependent_component_simple_file() {
    // Create a temporary file with simple JSON content
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    write!(temp_file, "{}", create_simple_json_content()).expect("Failed to write to temp file");

    let file_path = temp_file.path().to_string_lossy().to_string();
    let link = Link::FileLink { path: file_path };
    let mut context = create_test_context();

    let result = load_dependent_component(link, &mut context);

    match result {
        Ok(fragments) => {
            assert_eq!(fragments.len(), 1);
            assert_eq!(fragments[0].root.domains.len(), 1);
            assert_eq!(fragments[0].root.domains[0].domain_name, "test_domain");
            // Verify dependencies are voided
            assert!(fragments[0].root.take_from.is_empty());
        }
        Err(e) => panic!("Expected success but got error: {}", e),
    }
}

#[test]
fn test_load_dependent_component_empty_dependencies() {
    let content = r#"{
        "domains": [
            {
                "domain_name": "isolated_domain",
                "domain_code": 1,
                "components": []
            }
        ]
    }"#;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    write!(temp_file, "{}", content).expect("Failed to write to temp file");

    let file_path = temp_file.path().to_string_lossy().to_string();
    let link = Link::FileLink { path: file_path };
    let mut context = create_test_context();

    let result = load_dependent_component(link, &mut context);

    match result {
        Ok(fragments) => {
            assert_eq!(fragments.len(), 1);
            assert_eq!(fragments[0].root.domains.len(), 1);
            assert_eq!(fragments[0].root.domains[0].domain_name, "isolated_domain");
            assert!(fragments[0].root.take_from.is_empty());
        }
        Err(e) => panic!(
            "Expected success for empty dependencies but got error: {}",
            e
        ),
    }
}

#[test]
fn test_load_dependent_component_missing_file() {
    let link = Link::FileLink {
        path: "/nonexistent/file.json".to_string(),
    };
    let mut context = create_test_context();

    let result = load_dependent_component(link, &mut context);

    match result {
        Ok(_) => panic!("Expected error for missing file"),
        Err(LoadError::IOError { path: _, inner: _ }) => {
            // Expected error
        }
        Err(e) => panic!("Expected IO error but got: {}", e),
    }
}
