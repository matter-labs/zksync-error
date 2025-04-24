use zksync_error_model::error::ModelValidationError;
use zksync_error_model::link::Link;
use zksync_error_model::link::error::LinkError;

use crate::{description::merge::error::MergeError, loader::error::LoadError};

#[derive(Debug, thiserror::Error)]
#[error("Missing component {component_name} in the domain {domain_name}")]
pub struct MissingComponent {
    pub domain_name: String,
    pub component_name: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Missing domain {domain_name}")]
pub struct MissingDomain {
    pub domain_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ModelBuildingError {
    #[error("Error merging description {origin}: {inner}")]
    MergeError {
        inner: Box<MergeError>,
        origin: Link,
    },
    #[error("Error validating combined model: {0}")]
    ModelValidationError(#[from] ModelValidationError),
    #[error(transparent)]
    LoadError(#[from] LoadError),
}
