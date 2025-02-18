use zksync_error_model::link::error::LinkError;
use zksync_error_model::link::Link;

use super::builder::error::ModelBuildingError;
use super::resolution::error::ResolutionError;

#[derive(Debug, thiserror::Error)]
pub enum FileFormatError {
    #[error("File `{origin}` contains just an error domain description, but a master error database should describe at least one domain and one component.")]
    ExpectedFullGotDomain { origin: Link },
    #[error("File `{origin}` contains just an error component description, but a master error database should describe at least one domain and one component.")]
    ExpectedFullGotComponent { origin: Link },
    #[error("File `{origin}` contains just an array of errors, but a master error database should describe at least one domain and one component.")]
    ExpectedFullGotErrors { origin: Link },
    #[error(
        "Error parsing error description in JSON file.
{inner}
Note that the line number/column may be reported incorrectly.
{contents}"
    )]
    ParseError {
        contents: String,
        #[source]
        inner: Box<dyn std::error::Error>,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),

    #[error(transparent)]
    FileFormatError(#[from] FileFormatError),

    #[error(transparent)]
    LinkError(#[from] LinkError),

    #[error(transparent)]
    ResolutionError(#[from] ResolutionError),

    #[error("Missing file {0}")]
    MissingFileError(String),

    #[error(transparent)]
    ModelBuildingError(/* from */ Box<ModelBuildingError>), // Can not derive `From` here because of the `Box`
}

impl From<ModelBuildingError> for LoadError {
    fn from(v: ModelBuildingError) -> Self {
        Self::ModelBuildingError(Box::new(v))
    }
}
