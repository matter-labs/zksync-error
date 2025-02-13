use std::path::PathBuf;

use crate::backend::rust::error::GenerationError;
use crate::backend::rust::{RustBackend, RustBackendConfig};
use crate::backend::File;

impl RustBackend {
    pub fn generate_file_cargo(&mut self) -> Result<Option<File>, GenerationError> {
        if !self.config.generate_cargo_toml {
            return Ok(None);
        }

        let import_anyhow = if self.config.use_anyhow {
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

        Ok(Some(File {
            content,
            relative_path: PathBuf::from("Cargo.toml"),
        }))
    }
}
