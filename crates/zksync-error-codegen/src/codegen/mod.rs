pub mod file;
pub mod mdbook;
pub mod rust;

use file::File;

pub trait IBackendConfig {}

pub trait Backend
{
    type Error;
    type Config: IBackendConfig;
    fn get_name() -> &'static str;
    fn get_language_name() -> &'static str;
    fn generate(&mut self, config: &Self::Config) -> Result<Vec<File>, Self::Error>;

}
