use crate::description::{Domain, HierarchyFragmentKind};
use crate::util::printing::vec_debug;
use zksync_error_model::inner::{component, domain};

use super::Component;

#[derive(Debug, thiserror::Error)]
pub enum FileFormatError {
    #[error("File contains {got:?}, but expected {expected:?}.")]
    UnexpectedFormat {
        expected: Vec<HierarchyFragmentKind>,
        got: HierarchyFragmentKind,
    },
    #[error(
        "Error parsing error description:
{inner}

Note that the line number/column may be reported incorrectly.

{contents}"
    )]
    ParseError {
        contents: String,
        #[source]
        inner: Box<dyn std::error::Error>,
    },
    #[error("No domains matching identifier {expected}")]
    NoDomains { expected: domain::PartialIdentifier },
    #[error("Multiple domains matching {expected}: {}", vec_debug(domains, "\n"))]
    MultipleDomains {
        expected: domain::PartialIdentifier,
        domains: Vec<Domain>,
    },
    #[error("Domain name or code in {} does not match with {expected}", Into::<domain::PartialIdentifier>::into(domain))]
    WrongDomain {
        expected: domain::PartialIdentifier,
        domain: Domain,
    },

    #[error("No components matching identifier {expected}")]
    NoComponents {
        expected: component::PartialIdentifier,
    },

    #[error(
        "Multiple components matching {expected}: {}",
        vec_debug(components, "\n")
    )]
    MultipleComponents {
        expected: component::PartialIdentifier,
        components: Vec<Component>,
    },

    #[error("Component name or code in {} does not match with {expected}", Into::<component::PartialIdentifier>::into(component))]
    WrongComponent {
        expected: component::PartialIdentifier,
        component: Component,
    },
}
