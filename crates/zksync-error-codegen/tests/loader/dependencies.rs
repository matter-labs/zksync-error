use std::fs;
use tempfile::TempDir;
use zksync_error_codegen::loader::load_dependent_component;
use zksync_error_model::link::Link;

use super::common::*;

#[test]
fn test_load_dependent_component_with_dependencies() {
    // Create temporary directory
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create dependency file
    let dependency_path = temp_dir.path().join("dependency1.json");
    fs::write(&dependency_path, create_simple_json_content())
        .expect("Failed to write dependency file");

    // Create main file with dependency
    let main_content = format!(
        r#"{{
        "take_from": [
            "file://{}"
        ],
        "domains": [
            {{
                "domain_name": "main_domain",
                "domain_code": 1,
                "components": []
            }}
        ]
    }}"#,
        dependency_path.to_string_lossy()
    );

    let main_path = temp_dir.path().join("main.json");
    fs::write(&main_path, main_content).expect("Failed to write main file");

    let link = Link::FileLink {
        path: main_path.to_string_lossy().to_string(),
    };
    let mut context = create_test_context();

    let result = load_dependent_component(link, &mut context);

    match result {
        Ok(fragments) => {
            assert_eq!(fragments.len(), 2); // Main + dependency
            // Verify both fragments have voided dependencies
            for fragment in &fragments {
                assert!(fragment.root.take_from.is_empty());
            }
        }
        Err(e) => panic!("Expected success but got error: {}", e),
    }
}

#[test]
fn test_load_dependent_component_complex_dependency_chain() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a chain: main -> dep1 -> dep2
    let dep2_path = temp_dir.path().join("dep2.json");
    let dep1_path = temp_dir.path().join("dep1.json");
    let main_path = temp_dir.path().join("main.json");

    fs::write(&dep2_path, create_simple_json_content()).expect("Failed to write dep2");

    let dep1_content = format!(
        r#"{{
        "take_from": [
            "file://{}"
        ],
        "domains": [
            {{
                "domain_name": "dep1_domain",
                "domain_code": 1,
                "components": []
            }}
        ]
    }}"#,
        dep2_path.to_string_lossy()
    );
    fs::write(&dep1_path, dep1_content).expect("Failed to write dep1");

    let main_content = format!(
        r#"{{
        "take_from": [
            "file://{}"
        ],
        "domains": [
            {{
                "domain_name": "main_domain",
                "domain_code": 2,
                "components": []
            }}
        ]
    }}"#,
        dep1_path.to_string_lossy()
    );
    fs::write(&main_path, main_content).expect("Failed to write main");

    let link = Link::FileLink {
        path: main_path.to_string_lossy().to_string(),
    };
    let mut context = create_test_context();

    let result = load_dependent_component(link, &mut context);

    match result {
        Ok(fragments) => {
            assert_eq!(fragments.len(), 3); // main + dep1 + dep2
            // Verify all dependencies are voided
            for fragment in &fragments {
                assert!(fragment.root.take_from.is_empty());
            }
        }
        Err(e) => panic!("Expected success with complex chain but got error: {}", e),
    }
}

#[test]
fn test_void_dependencies_functionality() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create dependency files
    let dep1_path = temp_dir.path().join("dependency1.json");
    let dep2_path = temp_dir.path().join("dependency2.json");

    fs::write(&dep1_path, create_simple_json_content()).expect("Failed to write dependency1 file");
    fs::write(&dep2_path, create_simple_json_content()).expect("Failed to write dependency2 file");

    // Create main file with multiple root-level dependencies
    let main_content = format!(
        r#"{{
        "take_from": [
            "file://{}",
            "file://{}"
        ],
        "domains": [
            {{
                "domain_name": "main_domain",
                "domain_code": 1,
                "components": [
                    {{
                        "component_name": "main_component",
                        "component_code": 1
                    }}
                ]
            }}
        ]
    }}"#,
        dep1_path.to_string_lossy(),
        dep2_path.to_string_lossy()
    );

    let main_path = temp_dir.path().join("main.json");
    fs::write(&main_path, main_content).expect("Failed to write main file");

    let link = Link::FileLink {
        path: main_path.to_string_lossy().to_string(),
    };
    let mut context = create_test_context();

    let result = load_dependent_component(link, &mut context);

    match result {
        Ok(fragments) => {
            // Should have 3 fragments: main + 2 dependencies
            assert_eq!(fragments.len(), 3);

            // Verify all take_from fields are voided at all levels
            for fragment in &fragments {
                assert!(
                    fragment.root.take_from.is_empty(),
                    "Root take_from should be empty after loading"
                );
                for domain in &fragment.root.domains {
                    assert!(
                        domain.take_from.is_empty(),
                        "Domain take_from should be empty after loading"
                    );
                    for component in &domain.components {
                        assert!(
                            component.take_from.is_empty(),
                            "Component take_from should be empty after loading"
                        );
                    }
                }
            }
        }
        Err(e) => panic!("Expected success but got error: {}", e),
    }
}
