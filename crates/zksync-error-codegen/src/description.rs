//!
//! Layout of the JSON file that holds a fragment of error hierarchy.
//!

#![allow(non_snake_case)]

use serde::Deserialize;

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

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ErrorNameMapping {
    pub rust: Option<ErrorType>,
    pub typescript: Option<ErrorType>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TypeMappings {
    pub rust: Option<FullyQualifiedType>,
    #[serde(default)]
    pub typescript: Option<FullyQualifiedType>,
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

#[derive(Clone, Debug, Default, Deserialize)]
pub struct NameBindings {
    pub rust: Option<String>,
    pub typescript: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Domain {
    pub domain_name: String,
    pub domain_code: u32,
    pub identifier_encoding: Option<String>,
    pub description: Option<String>,
    pub components: Vec<Component>,
    #[serde(default)]
    pub bindings: NameBindings,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Component {
    pub component_name: String,
    pub component_code: u32,

    pub identifier_encoding: Option<String>,
    pub description: Option<String>,

    #[serde(default)]
    pub bindings: NameBindings,
    #[serde(default)]
    pub takeFrom: Vec<String>,

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

impl Root {
    pub fn get_component(&self, domain: &str, component: &str) -> Option<&Component> {
        let domain = self.domains.iter().find(|d| d.domain_name == domain)?;
        let component = domain
            .components
            .iter()
            .find(|c| c.component_name == component)?;

        Some(component)
    }
}

impl Domain {
    pub fn get_component(&self, component: &str) -> Option<&Component> {
        self.components
            .iter()
            .find(|c| c.component_name == component)
    }
}

impl Collection {
    pub fn get_component(&self, domain: &str, component_name: &str) -> Option<&Component> {
        match self {
            Collection::Root(root) => root.get_component(domain, component_name),
            Collection::Domain(domain) => domain.get_component(component_name),
            Collection::Component(component) if component.component_name == component_name => {
                Some(component)
            }
            _ => None,
        }
    }
    pub fn get_component_errors(&self, domain: &str, component_name: &str) -> Option<&Vec<Error>> {
        match self {
            Collection::Root(root) => root
                .get_component(domain, component_name)
                .map(|c| &c.errors),
            Collection::Domain(domain) => domain.get_component(component_name).map(|c| &c.errors),
            Collection::Component(component) if component.component_name == component_name => {
                Some(&component.errors)
            }
            Collection::Errors(errors) => Some(errors),
            _ => None,
        }
    }
}
