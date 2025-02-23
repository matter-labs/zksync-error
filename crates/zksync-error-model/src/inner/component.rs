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

#[derive(
    Debug,
    derive_more::Display,
    Eq,
    PartialEq,
    Clone,
    serde::Deserialize,
    serde::Serialize,
    Ord,
    PartialOrd,
)]
#[display("{name} (code: {code})")]
pub struct PartialIdentifier {
    pub name: ComponentName,
    pub code: ComponentCode,
}

impl From<&Identifier> for PartialIdentifier {
    fn from(value: &Identifier) -> Self {
        let Identifier { name, code, .. } = value;
        Self {
            name: name.clone(),
            code: *code,
        }
    }
}
