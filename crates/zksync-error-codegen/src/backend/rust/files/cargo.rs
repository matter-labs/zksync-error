use std::path::PathBuf;

use crate::backend::File;
use crate::backend::rust::error::GenerationError;
use crate::backend::rust::{RustBackend, RustBackendConfig};

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

        let preamble = RustBackendConfig::PREAMBLE;
        let content = format!(
            r#"######################################
# {preamble}
######################################

[package]
name = "zksync_error"
version = "0.1.0"
edition = "2021"

[dependencies]
lazy_static = "1.5.0"
serde = {{ version = "1.0.210", features = [ "derive", "rc" ] }}
serde_json = "1.0.128"
strum = "0.26.3"
strum_macros = "0.26.4"

# Required for types such as H160, H256, and U256.
zksync_basic_types = {{ git = "https://github.com/matter-labs/zksync-era", tag = "core-v26.4.0" }}

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
