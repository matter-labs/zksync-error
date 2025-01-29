use crate::codegen::IBackendConfig;

pub struct Config {
    pub use_anyhow: bool,
}

impl IBackendConfig for Config {}
