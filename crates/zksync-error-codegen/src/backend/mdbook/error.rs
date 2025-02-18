use zksync_error_model::error::ModelValidationError;

#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error(transparent)]
    ModelError(#[from] ModelValidationError),
    #[error(transparent)]
    TemplateError(#[from] tera::Error),
}
