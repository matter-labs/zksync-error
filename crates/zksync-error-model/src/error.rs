#![allow(unreachable_patterns)]

#[derive(Debug, thiserror::Error)]
pub enum ModelValidationError {
    #[error("Unknown model type {0}. Ensure the \"types\" object of the error definitions file contains it.")]
    UnknownType(String),
    #[error("Type {0} has no mappings for the Rust backend.")]
    UnmappedType(String),
    #[error("The name {0} has no mapping.")]
    UnmappedName(String),
}
