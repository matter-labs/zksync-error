use crate::loader::builder::error::ModelBuildingError;
use crate::loader::error::LoadError;
use zksync_error_model::error::ModelValidationError;
use zksync_error_model::link::error::LinkError;

#[derive(Debug, thiserror::Error)]
pub enum ProgramError {
    #[error(transparent)]
    ModelError(#[from] ModelValidationError),
    #[error(transparent)]
    ModelBuildingError(#[from] ModelBuildingError),
    #[error(transparent)]
    JsonDeserializationError(#[from] serde_json::Error),
    #[error("Error in backend {backend_name}: {inner}")]
    BackendError {
        backend_name: String,
        inner: Box<dyn std::error::Error>,
    },
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    LoadError(#[from] LoadError),
    #[error(transparent)]
    LinkError(#[from] LinkError),
}
