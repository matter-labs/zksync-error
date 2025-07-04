use crate::{
    arguments::ResolutionMode,
    loader::{dependency_lock::DependencyLock, error::LoadError},
};

use super::overrides::Remapping;

#[derive(Clone, Debug)]
pub enum ResolutionContext {
    /// Overrides are applied, lockfile is ignored
    NoLock { overrides: Remapping },
    /// - Overrides are applied
    /// - Links are resolved using the lockfile
    /// - Links absent from lockfile are resolved in `NoLock` mode and added to
    ///   the lockfile
    LockOrPopulate {
        overrides: Remapping,
        lock: DependencyLock,
    },
    /// - No overrides
    /// - Github links are resolved using the lockfile only
    /// - Github links that are missing from lockfile can not be resolved
    LockOnly { lock: DependencyLock },
}

impl TryFrom<&ResolutionMode> for ResolutionContext {
    type Error = LoadError;

    fn try_from(value: &ResolutionMode) -> Result<Self, Self::Error> {
        Ok(match value {
            ResolutionMode::NoLock { override_links } => ResolutionContext::NoLock {
                overrides: Remapping::try_from(override_links)?,
            },
            ResolutionMode::Normal {
                override_links,
                lock_file,
            } => ResolutionContext::LockOrPopulate {
                overrides: Remapping::try_from(override_links)?,
                lock: DependencyLock::load_from_file_or_create(lock_file),
            },
            ResolutionMode::Reproducible { lock_file } => ResolutionContext::LockOnly {
                lock: DependencyLock::load_from_file(lock_file)?,
            },
        })
    }
}
