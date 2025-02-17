use super::ErrorCode;
use super::ErrorName;

#[derive(
    Debug, derive_more::Display, Eq, PartialEq, Clone, serde::Deserialize, serde::Serialize,
)]
#[display("{name} (code: {code})")]
pub struct Identifier {
    pub name: ErrorName,
    pub code: ErrorCode,
}
