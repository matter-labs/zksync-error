use std::path::PathBuf;

use super::builder::error::ModelBuildingError;
use super::resolution::error::ResolutionError;
use crate::description::error::FileFormatError;
use zksync_error_model::link::error::LinkError;
use zksync_error_model::link::Link;

#[derive(Debug, thiserror::Error)]
pub enum TakeFromError {
    #[error("Error while building model following a `take_from` link: {0}")]
    LoadError(#[from] LoadError),

    #[error("Error while building model following a `take_from` link: {0}")]
    LinkError(#[from] LinkError),

    #[error("Circular dependency detected: file {trigger} attempted to reference {visited} which was already visited.")]
    CircularDependency { trigger: Link, visited: Link },
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("Error loading file from {path}: {inner}")]
    IOError {
        path: PathBuf,
        inner: std::io::Error,
    },

    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),

    #[error("Error loading errors from {origin}: {inner}")]
    FileFormatError {
        origin: Link,
        inner: FileFormatError,
    },

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
