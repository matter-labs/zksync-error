use crate::identifier::ErrorIdentifier;
use crate::inner::{
    ComponentCode, ComponentName, DomainCode, DomainName, ErrorCode, ErrorMessageTemplate,
    ErrorName, FieldName, LanguageName, Model, Semver, TypeName,
};
use std::collections::BTreeMap;

type ErrorIdentifierRepr = String;

#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct TargetLanguageType {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeMetadata {
    pub description: String,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeDescription {
    pub name: TypeName,
    pub meta: TypeMetadata,
    pub bindings: BTreeMap<LanguageName, TargetLanguageType>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct DomainMetadata {
    pub name: DomainName,
    pub code: DomainCode,
    pub components: Vec<ComponentName>,
    pub bindings: BTreeMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnpackedModel {
    pub types: BTreeMap<TypeName, TypeDescription>,
    pub domains: BTreeMap<DomainName, DomainMetadata>,
    pub components: BTreeMap<ComponentName, ComponentMetadata>,
    pub errors: BTreeMap<ErrorIdentifierRepr, ErrorDescription>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComponentMetadata {
    pub name: ComponentName,
    pub code: ComponentCode,
    pub domain_name: DomainName,
    pub bindings: BTreeMap<LanguageName, String>,
    pub identifier: String,
    pub description: String,
}

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

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldDescription {
    pub name: FieldName,
    pub r#type: TypeName,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorDocumentation {
    pub description: String,
    pub summary: String,
    pub likely_causes: Vec<LikelyCause>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct LikelyCause {
    pub cause: String,
    pub fixes: Vec<String>,
    pub report: String,
    pub owner: Option<VersionedOwner>,
    pub references: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionedOwner {
    pub name: String,
    pub version: Semver,
}

fn translate_domain_metadata(
    meta: &crate::inner::DomainMetadata,
    components: Vec<ComponentName>,
) -> DomainMetadata {
    let crate::inner::DomainMetadata {
        name,
        code,
        bindings,
        identifier,
        description,
    } = meta.clone();
    DomainMetadata {
        name,
        code,
        bindings,
        identifier,
        description,
        components,
    }
}

fn translate_component_metadata(
    meta: &crate::inner::ComponentMetadata,
    domain_name: &str,
) -> ComponentMetadata {
    let crate::inner::ComponentMetadata {
        name,
        code,
        bindings,
        identifier,
        description,
    } = meta.clone();
    ComponentMetadata {
        name,
        code,
        bindings,
        identifier,
        description,
        domain_name: domain_name.to_string(),
    }
}
fn translate_field(field: &crate::inner::FieldDescription) -> FieldDescription {
    let crate::inner::FieldDescription { name, r#type } = field.clone();
    FieldDescription { name, r#type }
}
fn translate_error(meta: &crate::inner::ErrorDescription) -> ErrorDescription {
    let crate::inner::ErrorDescription {
        domain,
        component,
        name,
        code,
        message,
        fields,
        documentation,
        bindings,
    } = meta;
    let new_bindings: BTreeMap<_, _> = bindings
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                TargetLanguageType {
                    name: v.name.to_string(),
                    path: String::default(),
                },
            )
        })
        .collect();
    let identifier = ErrorIdentifier {
        domain: domain.identifier.clone(),
        component: component.identifier.clone(),
        code: *code,
    }
    .to_string();
    ErrorDescription {
        domain: domain.name.clone(),
        component: component.name.clone(),
        name: name.clone(),
        code: *code,
        identifier,
        message: message.clone(),
        fields: fields.iter().map(translate_field).collect(),
        documentation: documentation.clone().map(|d| translate_documentation(&d)),
        bindings: new_bindings,
    }
}

fn translate_owner(doc: &Option<crate::inner::VersionedOwner>) -> Option<VersionedOwner> {
    if let Some(crate::inner::VersionedOwner { name, version }) = doc.clone() {
        Some(VersionedOwner { name, version })
    } else {
        None
    }
}
fn translate_likely_cause(doc: &crate::inner::LikelyCause) -> LikelyCause {
    let crate::inner::LikelyCause {
        cause,
        fixes,
        report,
        owner,
        references,
    } = doc.clone();

    LikelyCause {
        cause,
        fixes,
        report,
        owner: translate_owner(&owner),
        references,
    }
}
fn translate_documentation(doc: &crate::inner::ErrorDocumentation) -> ErrorDocumentation {
    let crate::inner::ErrorDocumentation {
        description,
        summary,
        likely_causes,
    } = doc.clone();

    ErrorDocumentation {
        description,
        summary: summary.unwrap_or_default(),
        likely_causes: likely_causes.iter().map(translate_likely_cause).collect(),
    }
}

fn translate_type(typ: &crate::inner::TypeDescription) -> TypeDescription {
    let crate::inner::TypeDescription {
        name,
        meta: crate::inner::TypeMetadata { description },
        bindings,
    } = typ.clone();

    let new_bindings: BTreeMap<_, _> = bindings
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                TargetLanguageType {
                    name: v.name.to_string(),
                    path: String::default(),
                },
            )
        })
        .collect();
    TypeDescription {
        name,
        meta: TypeMetadata { description },
        bindings: new_bindings,
    }
}
pub fn flatten(model: &Model) -> UnpackedModel {
    let Model { types, domains } = model;
    let mut result = UnpackedModel::default();
    for (name, typ) in types {
        result.types.insert(name.clone(), translate_type(typ));
    }

    for (domain_name, crate::inner::DomainDescription { meta, components }) in domains {
        let component_names: Vec<_> = components.keys().cloned().collect();
        result.domains.insert(
            domain_name.to_string(),
            translate_domain_metadata(meta, component_names),
        );
        result.components.extend(components.iter().map(|(n, c)| {
            (
                n.to_string(),
                translate_component_metadata(&c.meta, domain_name.as_str()),
            )
        }));

        for component in components.values() {
            result.errors.extend(
                component
                    .errors
                    .iter()
                    .map(|e| (e.get_identifier().to_string(), translate_error(e))),
            )
        }
    }

    result
}
