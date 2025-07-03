use std::fs;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};
use zksync_error_codegen::loader::{error::LoadError, load_dependent_component};
use zksync_error_model::link::Link;

use super::common::*;

#[test]
fn test_load_dependent_component_circular_dependency() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create files that reference each other
    let file1_path = temp_dir.path().join("file1.json");
    let file2_path = temp_dir.path().join("file2.json");

    let file1_content = format!(
        r#"{{
        "take_from": [
            "file://{}"
        ],
        "domains": [
            {{
                "domain_name": "domain1",
                "domain_code": 1,
                "components": []
            }}
        ]
    }}"#,
        file2_path.to_string_lossy()
    );

    let file2_content = format!(
        r#"{{
        "take_from": [
            "file://{}"
        ],
        "domains": [
            {{
                "domain_name": "domain2",
                "domain_code": 2,
                "components": []
            }}
        ]
    }}"#,
        file1_path.to_string_lossy()
    );

    fs::write(&file1_path, file1_content).expect("Failed to write file1");
    fs::write(&file2_path, file2_content).expect("Failed to write file2");

    let link = Link::FileLink {
        path: file1_path.to_string_lossy().to_string(),
    };
    let mut context = create_test_context();

    let result = load_dependent_component(link, &mut context);

    match result {
        Ok(_) => panic!("Expected circular dependency error"),
        Err(LoadError::CircularDependency {
            trigger: _,
            visited: _,
        }) => {
            // Expected error
        }
        Err(e) => panic!("Expected circular dependency error but got: {e}"),
    }
}

#[test]
fn test_load_dependent_component_invalid_json() {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    write!(temp_file, "invalid json content").expect("Failed to write to temp file");

    let file_path = temp_file.path().to_string_lossy().to_string();
    let link = Link::FileLink { path: file_path };
    let mut context = create_test_context();

    let result = load_dependent_component(link.clone(), &mut context);

    match result {
        Ok(_) => panic!("Expected error for invalid JSON"),
        Err(LoadError::FileFormatError { origin, inner: _ }) => {
            assert_eq!(origin, link);
        }
        Err(e) => panic!("Expected file format error but got: {e}"),
    }
}

#[test]
fn test_load_dependent_component_invalid_link_format() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let main_path = temp_dir.path().join("main.json");

    let content_with_invalid_link = r#"{
        "take_from": [
            "invalidprotocol://some-path"
        ],
        "domains": []
    }"#;

    fs::write(&main_path, content_with_invalid_link).expect("Failed to write main file");

    let link = Link::FileLink {
        path: main_path.to_string_lossy().to_string(),
    };
    let mut context = create_test_context();

    let result = load_dependent_component(link, &mut context);

    match result {
        Ok(_) => panic!("Expected error for invalid link format"),
        Err(LoadError::LinkError(_)) => {
            // Expected error
        }
        Err(e) => panic!("Expected link error but got: {e}"),
    }
}
