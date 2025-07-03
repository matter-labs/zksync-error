use std::collections::BTreeMap;
use zksync_error_codegen::loader::resolution::{context::ResolutionContext, overrides::Remapping};
use zksync_error_model::link::Link;

pub fn create_test_context() -> ResolutionContext {
    ResolutionContext::NoLock {
        overrides: Remapping {
            map: BTreeMap::new(),
        },
    }
}

pub fn create_test_context_with_overrides(overrides: BTreeMap<Link, Link>) -> ResolutionContext {
    ResolutionContext::NoLock {
        overrides: Remapping { map: overrides },
    }
}

// Mock data for testing
pub fn create_simple_json_content() -> String {
    r#"{
        "domains": [
            {
                "domain_name": "test_domain",
                "domain_code": 1,
                "components": []
            }
        ]
    }"#
    .to_string()
}
