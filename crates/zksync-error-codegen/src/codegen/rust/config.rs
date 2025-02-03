use crate::codegen::IBackendConfig;

pub struct Config {
    pub use_anyhow: bool,
}

impl IBackendConfig for Config {}
impl Config {
    pub const SHARED_MODEL_CRATE_URL: &str = r"https://github.com/matter-labs/zksync-error";
}
