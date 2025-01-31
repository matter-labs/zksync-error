use crate::loader::link::Link;

use super::ResolutionContext;

#[derive(Debug, thiserror::Error)]
pub enum ResolutionError {
    #[error("Failed to resolve `{link}` in context {context:?}.")]
    CargoLinkResolutionError {
        link: Link,
        context: ResolutionContext,
    },
    #[error("Failed to resolve `{link}`.")]
    GenericLinkResolutionError { link: Link },
}
