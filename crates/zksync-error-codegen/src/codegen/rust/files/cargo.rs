use std::path::PathBuf;

use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::{RustBackend, RustBackendConfig};
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_cargo(
        &mut self,
        config: &RustBackendConfig,
    ) -> Result<File, GenerationError> {
        let import_anyhow = if config.use_anyhow {
            r#"anyhow = "1.0""#
        } else {
            ""
        };
        let content = format!(
            r#"
[package]
name = "zksync_error"
version = "0.1.0"
edition = "2021"
[lib]

[dependencies]
lazy_static = "1.5.0"
serde = {{ version = "1.0.210", features = [ "derive", "rc" ] }}
serde_json = "1.0.128"
strum = "0.26.3"
strum_macros = "0.26.4"
zksync-error-description = {{ git = "{}", branch = "main"}}
{import_anyhow}
"#,
            RustBackendConfig::SHARED_MODEL_CRATE_URL,
        );

        Ok(File {
            content,
            relative_path: PathBuf::from("Cargo.toml"),
        })
    }
}
