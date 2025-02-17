use super::ComponentCode;
use super::ComponentName;

#[derive(
    Debug, derive_more::Display, Eq, PartialEq, Clone, serde::Deserialize, serde::Serialize,
)]
#[display("{name} (code: {code}, encoding: {encoding})")]
pub struct Identifier {
    pub name: ComponentName,
    pub code: ComponentCode,
    pub encoding: String,
}
