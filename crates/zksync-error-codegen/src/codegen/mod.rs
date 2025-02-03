pub mod file;
pub mod mdbook;
pub mod rust;

use file::File;

pub trait IBackendConfig {}

pub trait Backend<Config>
where
    Config: IBackendConfig,
{
    type Error;
    fn get_name() -> &'static str;
    fn get_language_name() -> &'static str;
    fn generate(&mut self, config: &Config) -> Result<Vec<File>, Self::Error>;
}
