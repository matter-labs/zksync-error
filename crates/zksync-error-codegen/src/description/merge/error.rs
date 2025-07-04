use crate::description::Component;
use crate::description::Domain;
use crate::description::Error;

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum MergeError {
    #[error("Duplicate bindings for the type `{0}`")]
    DuplicateTypeBinding(String),
    #[error("Conflicting descriptions for type `{0}`")]
    ConflictingTypeDescriptions(Box<MergeError>),
    #[error("Conflicting descriptions for domain `{0:?}`")]
    ConflictingDomainDefinitions(Box<Domain>, Box<Domain>),
    #[error("Expected strings `{0}` and `{1}` to be equal`")]
    StringsDiffer(String, String),
    #[error("Conflicting descriptions for component `{0:?}`")]
    ConflictingComponentDefinitions(Box<Component>, Box<Component>),
    #[error("Conflicting error descriptions for errors `{0}` and `{1}`")]
    ConflictingErrorDescriptions(Box<Error>, Box<Error>),
}
