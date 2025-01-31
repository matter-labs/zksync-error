use crate::codegen::mdbook::error::GenerationError as MarkdownGenerationError;
use crate::codegen::rust::error::GenerationError as RustGenerationError;
use crate::loader::builder::error::ModelBuildingError;
use crate::loader::error::{LinkError, LoadError};
use zksync_error_model::error::ModelValidationError;

#[derive(Debug, thiserror::Error)]
pub enum ProgramError {
    #[error(transparent)]
    ModelError(#[from] ModelValidationError),
    #[error(transparent)]
    ModelBuildingError(#[from] ModelBuildingError),
    #[error(transparent)]
    JsonDeserializationError(#[from] serde_json::Error),
    #[error(transparent)]
    RustGenerationError(#[from] RustGenerationError),
    #[error(transparent)]
    MarkdownGenerationError(#[from] MarkdownGenerationError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    LoadError(#[from] LoadError),
    #[error(transparent)]
    LinkError(#[from] LinkError),
}
