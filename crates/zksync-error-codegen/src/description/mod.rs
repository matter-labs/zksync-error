//!
//! Layout of the JSON file that holds a fragment of error hierarchy.
//!

pub mod accessors;

use std::collections::BTreeMap;

use serde::Deserialize;

pub type TypeMappings = BTreeMap<String, FullyQualifiedType>;
pub type ErrorNameMapping = BTreeMap<String, ErrorType>;

#[derive(Clone, Debug, Deserialize)]
pub struct Root {
    #[serde(default)]
    pub types: Vec<Type>,
    pub domains: Vec<Domain>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Type {
    pub name: String,
    pub description: String,
    pub bindings: TypeMappings,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ErrorType {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FullyQualifiedType {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Domain {
    pub domain_name: String,
    pub domain_code: u32,
    pub identifier_encoding: Option<String>,
    pub description: Option<String>,
    pub components: Vec<Component>,
    #[serde(default)]
    pub bindings: BTreeMap<String, String>,
    #[serde(default)]
    pub take_from: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Component {
    pub component_name: String,
    pub component_code: u32,

    pub identifier_encoding: Option<String>,
    pub description: Option<String>,

    #[serde(default)]
    pub bindings: BTreeMap<String, String>,
    #[serde(default)]
    pub take_from: Vec<String>,

    #[serde(default)]
    pub errors: Vec<Error>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Error {
    pub name: String,
    pub code: u32,
    pub message: String,
    #[serde(default)]
    pub fields: Vec<Field>,

    #[serde(default)]
    pub bindings: ErrorNameMapping,
    #[serde(default)]
    pub doc: Option<ErrorDocumentation>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Field {
    pub name: String,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ErrorDocumentation {
    pub description: String,
    pub summary: Option<String>,
    #[serde(default)]
    pub likely_causes: Vec<LikelyCause>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum LikelyCause {
    Simple(String),
    Structured(StructuredLikelyCause),
}

#[derive(Clone, Debug, Deserialize)]
pub struct StructuredLikelyCause {
    pub cause: String,
    pub fixes: Vec<String>,
    #[serde(default)]
    pub report: String,
    #[serde(default)]
    pub owner: Option<VersionedOwner>,
    #[serde(default)]
    pub references: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct VersionedOwner {
    pub name: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Collection {
    Root(Root),
    Domain(Domain),
    Component(Component),
    Errors(Vec<Error>),
}
