use crate::{arguments::ResolutionMode, loader::error::LoadError};

use super::overrides::Remapping;

#[derive(Clone, Debug)]
pub enum ResolutionContext {
    /// Overrides are applied
    NoLock { overrides: Remapping },
}

impl TryFrom<&ResolutionMode> for ResolutionContext {
    type Error = LoadError;

    fn try_from(value: &ResolutionMode) -> Result<Self, Self::Error> {
        Ok(match value {
            ResolutionMode::NoLock { override_links } => ResolutionContext::NoLock {
                overrides: Remapping::try_from(override_links)?,
            },
        })
    }
}
