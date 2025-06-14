use std::path::PathBuf;

use crate::backend::File;
use crate::backend::rust::error::GenerationError;
use crate::backend::rust::{RustBackend, RustBackendConfig};

impl RustBackend {
    pub fn generate_file_cargo(&mut self) -> Result<Option<File>, GenerationError> {
        if !self.config.generate_cargo_toml {
            return Ok(None);
        }

        let preamble = RustBackendConfig::PREAMBLE;
        let content = format!(
            r#"######################################
# {preamble}
######################################

[package]
name = "zksync_error"
version = "0.1.0"
edition = "2021"

[features]
default = ["std", "use_anyhow", "use_serde"]
std = [ "serde/std", "lazy_static/spin_no_std", "anyhow?/std", "strum/std"]
use_anyhow = ["dep:anyhow"]
use_serde = ["dep:serde"]
runtime_documentation = ["dep:serde", "dep:serde_json"]
serialized_errors = ["dep:serde", "dep:serde_json"]
packed_errors = ["use_serde"]

[dependencies]
lazy_static = {{ version = "1.5.0", default-features = false, optional = true }}
serde = {{ version = "1.0.210", features = [ "derive", "alloc" ], default-features = false, optional = true }}
serde_json = {{ version = "1.0.128", optional = true }}
strum = {{ version = "0.26.3", default-features = false, features = ["derive"] }}
strum_macros = {{ version = "0.26.4", default-features = false }}
zksync-error-description = {{ git = "{}", branch = "main", default-features = false }}

[dependencies.anyhow]
version = "1.0"
optional = true
default-features = false

[lib]
doctest = false

[build-dependencies]
zksync-error-codegen = {{ git = "https://github.com/matter-labs/zksync-error", branch = "main", default-features = true }}


"#,
            RustBackendConfig::SHARED_MODEL_CRATE_URL,
        );

        Ok(Some(File {
            content,
            relative_path: PathBuf::from("Cargo.example.toml"),
        }))
    }
}
