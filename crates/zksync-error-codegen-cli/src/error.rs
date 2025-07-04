use zksync_error_codegen::error::ProgramError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Invalid argument `{argument}`: {reason}")]
    InvalidArgument { argument: String, reason: String },
    #[error(transparent)]
    ProgramError(#[from] Box<ProgramError>),
}
