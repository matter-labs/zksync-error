pub mod arguments;
pub mod file;
pub mod mdbook;
pub mod rust;

use std::error::Error as StdError;

use file::File;

pub trait IBackendConfig {
    fn parse_arguments(
        args: impl Iterator<Item = (String, String)>,
    ) -> Result<Self, self::arguments::ArgumentError>
    where
        Self: Sized;
}

pub trait Backend {
    type Config: IBackendConfig;
    type GenerationError: StdError + 'static;

    fn get_name() -> &'static str;
    fn get_language_name() -> &'static str;

    fn generate(&mut self) -> Result<Vec<File>, Self::GenerationError>;
    fn new(config: Self::Config, model: &zksync_error_model::inner::Model) -> Self
    where
        Self: Sized;
}
