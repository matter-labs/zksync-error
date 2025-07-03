//! Error types for dependency lock operations.

use thiserror::Error;

/// Errors that can occur during dependency lock operations.
#[derive(Debug, Error)]
pub enum LockError {
    /// An I/O error occurred while reading or writing the lock file.
    ///
    /// This can happen when the lock file cannot be read (e.g., permission issues,
    /// file doesn't exist) or written (e.g., disk full, permission issues).
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// The lock file contains invalid JSON that cannot be parsed.
    ///
    /// This indicates the lock file is corrupted or was manually edited incorrectly.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// A required dependency is missing from the lock file.
    ///
    /// This error occurs in reproducible build mode when a dependency is needed
    /// but not found in the lock file. This usually means the lock file is
    /// out of date and needs to be regenerated or modified by hand.
    #[error("Missing dependency in lock file: {url}")]
    MissingDependency {
        /// The URL or identifier of the missing dependency
        url: String,
    },
}
