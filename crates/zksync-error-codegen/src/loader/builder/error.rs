use crate::loader::{
    error::{LinkError, LoadError},
    link::Link,
};
use zksync_error_model::{error::ModelValidationError, merger::error::MergeError};

#[derive(Debug, thiserror::Error)]
#[error("Missing component {component_name} in the domain {domain_name}")]
pub struct MissingComponent {
    pub domain_name: String,
    pub component_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum TakeFromError {
    #[error("Error while building model following a `takeFrom` link: {0}")]
    IOError(#[from] LoadError),

    #[error("Error while building model following a `takeFrom` link: {0}")]
    ParsingError(#[from] serde_json::Error),

    #[error("Error while building model following a `takeFrom` link: {0}")]
    MissingComponent(#[from] MissingComponent),

    #[error("Error while building model following a `takeFrom` link: {0}")]
    ModelBuildingError(/* from */ Box<ModelBuildingError>), // Can't derive `From` implementation because of `Box`.

    #[error("Error while merging with the error base fetched from `takeFrom` link: {0}")]
    MergeError(#[from] MergeError),

    #[error("Error while building model following a `takeFrom` link: {0}")]
    LinkError(#[from] LinkError),
}

impl From<ModelBuildingError> for TakeFromError {
    fn from(v: ModelBuildingError) -> Self {
        Self::ModelBuildingError(Box::new(v))
    }
}

impl TakeFromError {
    pub fn from_address(self, address: &str) -> ModelBuildingError {
        ModelBuildingError::TakeFrom {
            address: address.to_string(),
            inner: self,
        }
    }
}
#[derive(Debug, thiserror::Error)]
pub enum ModelBuildingError {
    #[error("Failed to import a file {address}: {inner}")]
    TakeFrom {
        address: String,
        #[source]
        inner: TakeFromError,
    },

    #[error(
        "Error merging models {main_model_origin} and {additional_model_origin}: {merge_error}"
    )]
    MergeError {
        merge_error: MergeError,
        main_model_origin: Link,
        additional_model_origin: Link,
    },
    #[error("Error validating combined model: {0}")]
    ModelValidationError(#[from] ModelValidationError),
    #[error(transparent)]
    LoadError(#[from] LoadError),
}
