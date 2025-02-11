#![allow(unreachable_patterns)]

use crate::inner::{ComponentMetadata, DomainMetadata, ErrorDescription};

#[derive(Debug, thiserror::Error)]
pub enum ModelValidationError {
    #[error("Unknown model type {0}. Ensure the \"types\" object of the error definitions file contains it.")]
    UnknownType(String),
    #[error("Type {0} has no mappings for the Rust backend.")]
    UnmappedType(String),
    #[error("The name {0} has no mapping.")]
    UnmappedName(String),
    #[error(
        "At least two domains are assigned the same code, name, or identifier.
Domains:
{0:#?}
{1:#?}
 "
    )]
    NonUniqueDomains(DomainMetadata, DomainMetadata),
    #[error(
        "At least two components are assigned the same code, name, or identifier.
Components:
{0:#?}
{1:#?}

Parent domain:
{2:#?}
 "
    )]
    NonUniqueComponents(ComponentMetadata, ComponentMetadata, DomainMetadata),
    #[error(
        "At least two errors of a single component are assigned the same name of code.
Errors:
{0:#?}
{1:#?}

Parent component:
{2:#?}

Parent domain:
{3:#?}
 "
    )]
    NonUniqueErrors(
        ErrorDescription,
        ErrorDescription,
        ComponentMetadata,
        DomainMetadata,
    ),
}
