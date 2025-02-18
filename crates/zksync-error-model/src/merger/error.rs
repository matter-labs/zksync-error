use crate::inner::component;
use crate::inner::domain;

#[derive(Debug, thiserror::Error)]
pub enum MergeError {
    #[error("Duplicate bindings for the type `{0}`")]
    DuplicateTypeBinding(String),
    #[error("Conflicting descriptions for type `{0}`")]
    ConflictingTypeDescriptions(String),
    #[error("Conflicting descriptions for domain `{0:?}`")]
    ConflictingDomainDefinitions(domain::Identifier),
    #[error("Expected strings `{0}` and `{1}` to be equal`")]
    StringsDiffer(String, String),
    #[error("Conflicting descriptions for component `{0:?}`")]
    ConflictingComponentDefinitions(component::Identifier),
    #[error("Conflicting error descriptions for errors `{0}` and `{1}`")]
    ConflictingErrorDescriptions(String, String),
}
