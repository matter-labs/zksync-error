use crate::codegen::html::error::GenerationError as HtmlGenerationError;
use crate::codegen::mdbook::error::GenerationError as MarkdownGenerationError;
use crate::codegen::rust::error::GenerationError as RustGenerationError;
use crate::loader::builder::error::ModelBuildingError;
use crate::loader::error::{LinkError, LoadError};
use zksync_error_model::error::ModelValidationError;

#[derive(Debug)]
pub enum ProgramError {
    ModelError(ModelValidationError),
    ModelBuildingError(ModelBuildingError),
    JsonDeserializationError(serde_json::Error),
    RustGenerationError(RustGenerationError),
    HtmlGenerationError(HtmlGenerationError),
    MarkdownGenerationError(MarkdownGenerationError),
    IOError(std::io::Error),
    LoadError(LoadError),
    LinkError(LinkError),
}

impl From<LinkError> for ProgramError {
    fn from(v: LinkError) -> Self {
        Self::LinkError(v)
    }
}

impl From<MarkdownGenerationError> for ProgramError {
    fn from(v: MarkdownGenerationError) -> Self {
        Self::MarkdownGenerationError(v)
    }
}

impl From<LoadError> for ProgramError {
    fn from(v: LoadError) -> Self {
        Self::LoadError(v)
    }
}

impl From<HtmlGenerationError> for ProgramError {
    fn from(v: HtmlGenerationError) -> Self {
        Self::HtmlGenerationError(v)
    }
}

impl From<ModelBuildingError> for ProgramError {
    fn from(v: ModelBuildingError) -> Self {
        Self::ModelBuildingError(v)
    }
}

impl From<serde_json::Error> for ProgramError {
    fn from(v: serde_json::Error) -> Self {
        Self::JsonDeserializationError(v)
    }
}

impl From<ModelValidationError> for ProgramError {
    fn from(v: ModelValidationError) -> Self {
        Self::ModelError(v)
    }
}

impl From<RustGenerationError> for ProgramError {
    fn from(v: RustGenerationError) -> Self {
        Self::RustGenerationError(v)
    }
}

impl From<std::io::Error> for ProgramError {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}
