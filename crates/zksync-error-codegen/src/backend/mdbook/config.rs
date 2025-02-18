use crate::backend::arguments::ArgumentError;
use crate::backend::IBackendConfig;

#[derive(Default)]
pub struct MDBookBackendConfig;

impl IBackendConfig for MDBookBackendConfig {
    fn parse_arguments(
        _args: impl Iterator<Item = (String, String)>,
    ) -> Result<Self, ArgumentError> {
        Ok(MDBookBackendConfig)
    }
}
