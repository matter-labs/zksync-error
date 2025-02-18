use super::DomainCode;
use super::DomainName;

#[derive(
    Debug, derive_more::Display, Eq, PartialEq, Clone, serde::Deserialize, serde::Serialize,
)]
#[display("{name} (code: {code}, encoding: {encoding})")]
pub struct Identifier {
    pub name: DomainName,
    pub code: DomainCode,
    pub encoding: String,
}
