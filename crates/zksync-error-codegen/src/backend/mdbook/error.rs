use zksync_error_model::error::ModelValidationError;

#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error(transparent)]
    ModelError(#[from] ModelValidationError),
    #[error("Error processing template for MDBook backend: {0:?}")]
    TemplateError(#[from] tera::Error),
}
