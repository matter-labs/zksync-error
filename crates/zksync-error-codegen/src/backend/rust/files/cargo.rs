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
            if self.config.no_std {
                r#"anyhow = { version = "1.0", default-features = false }"#
            } else {
                r#"anyhow = "1.0""#
            }
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

[features]
default = ["std"]
std = ["serde_json", "serde/std"]
runtime_documentation = []
serialized_errors = []
packed_errors = []

[dependencies]
lazy_static = {{ version = "1.5.0", features = ["spin_no_std"] }}
serde = {{ version = "1.0.210", features = [ "derive", "rc" ], default-features = false }}
serde_json = {{ version = "1.0.128", optional = true }}
strum = {{ version = "0.26.3", default-features = false }}
strum_macros = "0.26.4"

# Required for types such as H160, H256, and U256.
zksync_basic_types = {{ git = "https://github.com/matter-labs/zksync-era", tag = "core-v26.4.0", default-features = false }}

zksync-error-description = {{ git = "{}", branch = "main", default-features = false }}
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
