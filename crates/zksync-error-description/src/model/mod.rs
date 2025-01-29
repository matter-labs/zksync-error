use std::collections::BTreeMap;

pub type LanguageName = String;
pub type TypeName = String;
pub type FieldName = String;
pub type ComponentName = String;
pub type DomainName = String;
pub type ErrorName = String;
pub type ErrorIdentifierRepr = String;
pub type ErrorCode = u32;
pub type ComponentCode = u32;
pub type DomainCode = u32;
pub type ErrorMessageTemplate = String;
pub type Semver = String;

#[non_exhaustive]
#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct TargetLanguageType {
    pub name: String,
    pub path: String,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeMetadata {
    pub description: String,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeDescription {
    pub name: TypeName,
    pub meta: TypeMetadata,
    pub bindings: BTreeMap<LanguageName, TargetLanguageType>,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct DomainMetadata {
    pub name: DomainName,
    pub code: DomainCode,
    pub components: Vec<ComponentName>,
    pub bindings: BTreeMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
}
#[non_exhaustive]
#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorHierarchy {
    pub types: BTreeMap<TypeName, TypeDescription>,
    pub domains: BTreeMap<DomainName, DomainMetadata>,
    pub components: BTreeMap<ComponentName, ComponentMetadata>,
    pub errors: BTreeMap<ErrorIdentifierRepr, ErrorDescription>,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComponentMetadata {
    pub name: ComponentName,
    pub code: ComponentCode,
    pub domain_name: DomainName,
    pub bindings: BTreeMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorDescription {
    pub domain: DomainName,
    pub component: ComponentName,
    pub name: ErrorName,
    pub code: ErrorCode,
    pub identifier: String,
    pub message: ErrorMessageTemplate,
    pub fields: Vec<FieldDescription>,
    pub documentation: Option<ErrorDocumentation>,
    pub bindings: BTreeMap<LanguageName, TargetLanguageType>,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldDescription {
    pub name: FieldName,
    pub r#type: TypeName,
}

#[non_exhaustive]
#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorDocumentation {
    pub description: String,
    pub summary: String,
    pub likely_causes: Vec<LikelyCause>,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct LikelyCause {
    pub cause: String,
    pub fixes: Vec<String>,
    pub report: String,
    pub owner: Option<VersionedOwner>,
    pub references: Vec<String>,
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionedOwner {
    pub name: String,
    pub version: Semver,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WrappedErrorHierarchy {
    comment: Vec<String>,
    model: ErrorHierarchy,
}

impl ErrorHierarchy {
    pub fn wrap(self: ErrorHierarchy) -> WrappedErrorHierarchy {
        WrappedErrorHierarchy {
            comment: vec![
                "AUTOGENERATED FILE",
                "DO NOT EDIT MANUALLY",
                "The purpose of this file is to show the combined model, after all the links",
                "between models are resolved and the final model is constructed.",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            model: self,
        }
    }

    pub fn from_str(serialized_model: &str) -> ErrorHierarchy {
        let wrapped: WrappedErrorHierarchy =
            serde_json::from_str(serialized_model).expect("Always valid");
        wrapped.model
    }
}
