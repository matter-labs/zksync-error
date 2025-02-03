use zksync_error_model::error::ModelValidationError;

#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error(transparent)]
    ModelError(#[from] ModelValidationError),
    #[error(transparent)]
    ModelSerialization(#[from] serde_json::Error),
    #[error("rustfmt: {0:#?}")]
    FormatError(#[from] rustfmt_wrapper::Error),
}
