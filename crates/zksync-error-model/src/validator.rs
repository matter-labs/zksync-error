use super::error::ModelValidationError;
use super::inner::Model;

pub fn validate(_model: &Model) -> Result<(), ModelValidationError> {
    Ok(())
}
