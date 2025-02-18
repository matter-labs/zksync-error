use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum ArgumentError {
    #[error("Unable to parse boolean backend argument {argument} : {error}")]
    ParseBoolError {
        argument: String,
        error: std::str::ParseBoolError,
    },
    #[error("Unsupported backend argument {argument}")]
    UnsupportedArgument { argument: String },
    #[error("Invalid value {value} of argument {argument}: {message}")]
    InvalidArgument {
        value: String,
        argument: String,
        message: String,
    },
    #[error(transparent)]
    GenericError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub fn parse_bool(arg: &str, val: &str, dest: &mut bool) -> Result<(), ArgumentError> {
    *dest = FromStr::from_str(val).map_err(|error| ArgumentError::ParseBoolError {
        argument: arg.to_owned(),
        error,
    })?;
    Ok(())
}
