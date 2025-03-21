use crate::identifier::PublicErrorIdentifier;
use crate::inner::component::Identifier as ComponentIdentifier;
use crate::inner::domain::Identifier as DomainIdentifier;

use crate::inner::{
    ComponentName, DomainName, ErrorCode, ErrorMessageTemplate, ErrorName, FieldName, LanguageName,
    Model, Origins, Semver, TypeName,
};
use std::collections::BTreeMap;

type ErrorIdentifierRepr = String;

#[derive(Debug, Default, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct TargetLanguageType {
    pub expression: String,
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
    pub components: Vec<ComponentName>,
    pub bindings: BTreeMap<LanguageName, String>,
    pub identifier: DomainIdentifier,
    pub description: String,
    pub origins: Origins,
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
    pub domain_name: DomainName,
    pub bindings: BTreeMap<LanguageName, String>,
    pub identifier: ComponentIdentifier,
    pub description: String,
    pub origins: Origins,
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
    pub origins: Origins,
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
        bindings,
        identifier,
        description,
        origins,
    } = meta.clone();
    DomainMetadata {
        bindings,
        identifier,
        description,
        components,
        origins,
    }
}

fn translate_component_metadata(meta: &crate::inner::ComponentMetadata) -> ComponentMetadata {
    let crate::inner::ComponentMetadata {
        bindings,
        identifier,
        description,
        domain,
        origins,
    } = meta.clone();
    ComponentMetadata {
        bindings,
        identifier,
        description,
        domain_name: domain.identifier.name.to_string(),
        origins,
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
        origins,
    } = meta;
    let new_bindings: BTreeMap<_, _> = bindings
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                TargetLanguageType {
                    expression: v.expression.clone(),
                },
            )
        })
        .collect();
    let identifier = PublicErrorIdentifier {
        domain: domain.identifier.encoding.clone(),
        component: component.identifier.encoding.clone(),
        code: *code,
    }
    .to_string();
    ErrorDescription {
        domain: domain.identifier.name.clone(),
        component: component.identifier.name.clone(),
        name: name.clone(),
        code: *code,
        identifier,
        message: message.clone(),
        fields: fields.iter().map(translate_field).collect(),
        documentation: documentation.clone().map(|d| translate_documentation(&d)),
        bindings: new_bindings,
        origins: origins.clone(),
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
                    expression: v.expression.to_string(),
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
        result.components.extend(
            components
                .iter()
                .map(|(n, c)| (n.to_string(), translate_component_metadata(&c.meta))),
        );

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
