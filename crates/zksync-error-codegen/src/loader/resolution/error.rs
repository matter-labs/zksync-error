use crate::loader::link::Link;

use super::ResolutionContext;

#[derive(Debug)]
pub enum ResolutionError {
    CargoLinkResolutionError {
        link: Link,
        context: ResolutionContext,
    },
    GenericLinkResolutionError {
        link: Link,
    },
}

impl std::fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolutionError::CargoLinkResolutionError { link, context } => f.write_fmt(
                format_args!("Failed to resolve `{link}` in context {context:?}"),
            ),
            ResolutionError::GenericLinkResolutionError { link } => {
                f.write_fmt(format_args!("Failed to resolve `{link}`."))
            }
        }
    }
}
