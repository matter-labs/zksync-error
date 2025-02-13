use crate::backend::arguments::parse_bool;
use crate::backend::arguments::ArgumentError;
use crate::backend::IBackendConfig;

pub struct Config {
    pub use_anyhow: bool,
    pub generate_cargo_toml: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            use_anyhow: true,
            generate_cargo_toml: false,
        }
    }
}

impl IBackendConfig for Config {
    fn parse_arguments(
        args: impl Iterator<Item = (String, String)>,
    ) -> Result<Self, ArgumentError> {
        let mut config = Self::default();
        for (arg, val) in args {
            match arg.as_str() {
                "use_anyhow" => parse_bool(&arg, &val, &mut config.use_anyhow)?,
                "generate_cargo_toml" => parse_bool(&arg, &val, &mut config.generate_cargo_toml)?,
                _ => return Err(ArgumentError::UnsupportedArgument { argument: arg }),
            }
        }
        Ok(config)
    }
}
impl Config {
    pub const SHARED_MODEL_CRATE_URL: &str = r"https://github.com/matter-labs/zksync-error";
}
