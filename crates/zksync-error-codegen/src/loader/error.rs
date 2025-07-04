use std::path::PathBuf;

use super::{dependency_lock::error::LockError, resolution::error::ResolutionError};
use crate::description::{error::FileFormatError, parsers::link::LinkError};
use zksync_error_model::link::Link;

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("Error loading file from {path}: {inner}")]
    IOError {
        path: PathBuf,
        inner: std::io::Error,
    },

    #[error("Network error for {url}: {inner}")]
    NetworkError { url: String, inner: reqwest::Error },

    #[error("Error loading errors from {origin}: {inner}")]
    FileFormatError {
        origin: Link,
        inner: FileFormatError,
    },

    #[error(transparent)]
    LinkError(#[from] LinkError),

    #[error(transparent)]
    LockError(#[from] LockError),

    #[error(transparent)]
    ResolutionError(#[from] ResolutionError),

    #[error("Missing file {0}")]
    MissingFileError(String),

    #[error(
        "Circular dependency detected: file {trigger} attempted to reference {visited} which was already visited."
    )]
    CircularDependency { trigger: Link, visited: Link },

    #[error("Failed to import a file {address}: {inner}")]
    TakeFrom {
        address: Link,
        #[source]
        inner: Box<LoadError>,
    },
}
