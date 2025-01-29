use std::{collections::BTreeMap, rc::Rc};

use super::error::ModelValidationError;

pub type LanguageName = String;
pub type TypeName = String;
pub type FieldName = String;
pub type ComponentName = String;
pub type DomainName = String;
pub type ErrorName = String;
pub type ErrorCode = u32;
pub type ComponentCode = u32;
pub type DomainCode = u32;
pub type ErrorMessageTemplate = String;
pub type Semver = String;

#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize)]
pub struct FullyQualifiedTargetLanguageType {
    pub name: String,
    pub path: String,
}
#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize)]
pub struct TargetLanguageType {
    pub name: String,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct TypeMetadata {
    pub description: String,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct TypeDescription {
    pub name: TypeName,
    pub meta: TypeMetadata,
    pub bindings: BTreeMap<LanguageName, FullyQualifiedTargetLanguageType>,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize)]
pub struct Model {
    pub types: BTreeMap<TypeName, TypeDescription>,
    pub domains: BTreeMap<DomainName, DomainDescription>,
}

impl Model {
    pub fn new(
        types: BTreeMap<TypeName, TypeDescription>,
        domains: BTreeMap<DomainName, DomainDescription>,
    ) -> Self {
        Self { types, domains }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct DomainMetadata {
    pub name: DomainName,
    pub code: DomainCode,
    pub bindings: BTreeMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
}
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct DomainDescription {
    pub meta: Rc<DomainMetadata>,
    pub components: BTreeMap<ComponentName, ComponentDescription>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct ComponentMetadata {
    pub name: ComponentName,
    pub code: ComponentCode,
    pub bindings: BTreeMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct ComponentDescription {
    pub meta: Rc<ComponentMetadata>,
    pub errors: Vec<ErrorDescription>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct ErrorDescription {
    pub domain: Rc<DomainMetadata>,
    pub component: Rc<ComponentMetadata>,
    pub name: ErrorName,
    pub code: ErrorCode,
    pub message: ErrorMessageTemplate,
    pub fields: Vec<FieldDescription>,
    pub documentation: Option<ErrorDocumentation>,
    pub bindings: BTreeMap<LanguageName, TargetLanguageType>,
}

impl From<TargetLanguageType> for FullyQualifiedTargetLanguageType {
    fn from(value: TargetLanguageType) -> Self {
        Self::from(value.name.as_str())
    }
}

impl From<&str> for FullyQualifiedTargetLanguageType {
    fn from(value: &str) -> Self {
        FullyQualifiedTargetLanguageType {
            name: value.into(),
            path: "".into(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct FieldDescription {
    pub name: FieldName,
    pub r#type: TypeName,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize)]
pub struct ErrorDocumentation {
    pub description: String,
    pub summary: Option<String>,
    pub likely_causes: Vec<LikelyCause>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct LikelyCause {
    pub cause: String,
    pub fixes: Vec<String>,
    pub report: String,
    pub owner: Option<VersionedOwner>,
    pub references: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct VersionedOwner {
    pub name: String,
    pub version: Semver,
}

impl Model {
    pub fn get_type(
        &self,
        language: &str,
        name: &str,
    ) -> Result<&FullyQualifiedTargetLanguageType, ModelValidationError> {
        let type_description = self
            .types
            .get(name)
            .ok_or(ModelValidationError::UnknownType(name.to_string()))?;
        let mapped_type = type_description
            .bindings
            .get(language)
            .ok_or(ModelValidationError::UnmappedType(name.to_string()))?;
        Ok(mapped_type)
    }
}
