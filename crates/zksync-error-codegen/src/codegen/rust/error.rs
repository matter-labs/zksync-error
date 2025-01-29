use zksync_error_model::error::ModelValidationError;

#[derive(Debug)]
pub enum GenerationError {
    ModelError(ModelValidationError),
    ModelSerialization(serde_json::Error),
}

impl From<serde_json::Error> for GenerationError {
    fn from(v: serde_json::Error) -> Self {
        Self::ModelSerialization(v)
    }
}

impl std::fmt::Display for GenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:#?}"))
    }
}
impl std::error::Error for GenerationError {}

impl From<ModelValidationError> for GenerationError {
    fn from(value: ModelValidationError) -> Self {
        GenerationError::ModelError(value)
    }
}
