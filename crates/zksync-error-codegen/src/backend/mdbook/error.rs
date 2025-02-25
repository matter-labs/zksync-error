use zksync_error_model::error::ModelValidationError;

#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error(transparent)]
    ModelError(#[from] ModelValidationError),
    #[error("Tera library error: {0:?}")]
    TemplateError(#[from] tera::Error),
}
