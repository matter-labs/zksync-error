//! Basic dependency lock similar to Cargo.lock,
//! which enables reproducible builds by pinning resolved Github dependencies to
//! specific commits.

use error::LockError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use zksync_error_model::link::Link;

use super::resolution::ResolvedLink;

pub mod error;

/// A single dependency entry in the lock file.
///
/// This represents a mapping from an abstract link (a GitHub repository+path)
/// to its concrete resolved form (specific commit hash).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DependencyEntry {
    /// The original link as specified in the source
    pub link: Link,
    /// Resolved link.
    pub resolved: ResolvedLink,
}

/// The main dependency lock structure.
///
/// This holds all the dependency mappings and provides methods to load/save
/// the lock file, add new dependencies, and resolve existing ones.
///
/// # Lock File Format
///
/// The lock file is stored as JSON with the following structure:
/// ```json
/// {
///   "dependencies": [
///     {
///       "link": { "Github": { ... } },
///       "resolved": { ... }
///     }
///   ]
/// }
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DependencyLock {
    /// List of all dependency entries
    dependencies: Vec<DependencyEntry>,
}

impl DependencyLock {
    /// Creates a new empty dependency lock.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads a dependency lock from a file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the lock file to load
    ///
    /// # Returns
    ///
    /// Returns `Ok(DependencyLock)` if the file exists and is valid,
    /// or `Err(LockError)` if the file cannot be read or parsed.
    pub fn load_from_file<P: AsRef<Path> + std::fmt::Display>(path: P) -> Result<Self, LockError> {
        eprintln!("Loading lockfile from {path}:");
        let content = fs::read_to_string(path)?;
        eprintln!("Contents: {content}");
        let file: DependencyLock = serde_json::from_str(&content)?;
        Ok(file)
    }

    /// Loads a dependency lock from a file, or creates a new empty one if the file doesn't exist.
    ///
    /// This is a convenience method that never fails - if the lock file cannot be loaded
    /// (e.g., doesn't exist, is corrupted), it returns a new empty lock.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the lock file to load
    ///
    /// # Returns
    ///
    /// Always returns a `DependencyLock`, either loaded from file or newly created.
    pub fn load_from_file_or_create<P: AsRef<Path> + std::fmt::Display>(path: P) -> Self {
        let result = Self::load_from_file(path).unwrap_or_default();
        eprintln!("Lock:\n{result:?}");
        result
    }

    /// Saves the dependency lock to a file.
    ///
    /// The lock file is saved as pretty-printed JSON. If the file already exists,
    /// it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where to save the lock file
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or `Err(LockError)` if the file cannot be written.
    pub fn save_to_file<P: AsRef<Path> + std::fmt::Display>(
        &self,
        path: P,
    ) -> Result<(), LockError> {
        eprintln!("Saving lockfile to {path}");
        let content = serde_json::to_string_pretty(self)?;
        if path.as_ref().exists() {
            fs::remove_file(&path)?;
        }
        fs::write(&path, content)?;
        Ok(())
    }

    /// Adds a new dependency entry to the lock.
    ///
    /// If a dependency with the same link already exists, this method does nothing
    /// (no duplicate entries are created).
    pub fn add_dependency(&mut self, entry: DependencyEntry) {
        if self.get_dependency(&entry.link).is_none() {
            self.dependencies.push(entry);
        }
    }

    /// Finds a dependency entry for the given link.
    ///
    /// This method searches through all dependencies and returns the first one
    /// that matches the query link. The matching logic handles different link
    /// types appropriately (e.g., GitHub links use "loose" matching).
    ///
    /// # Arguments
    ///
    /// * `query` - The link to search for
    ///
    /// # Returns
    ///
    /// Returns `Some(&DependencyEntry)` if a matching dependency is found,
    /// or `None` if no match exists.
    pub fn get_dependency(&self, query: &Link) -> Option<&DependencyEntry> {
        let result = self
            .dependencies
            .iter()
            .find(|x| links_equivalent(&x.link, query));

        if let Some(resolved) = result {
            eprintln!(
                "Resolved {query} to {:?} through the lock file",
                resolved.resolved
            );
        }

        result
    }

    /// Determines whether a link should be included in the lock file.
    ///
    /// Currently, only GitHub links are locked because they can change over time
    /// (branches can move, default branches can change). Local file links and
    /// bundled resources are considered stable and don't need locking.
    ///
    /// # Returns
    ///
    /// Returns `true` if the link should be locked, `false` otherwise.
    pub fn should_lock(link: &Link) -> bool {
        matches!(link, Link::Github(_))
    }
}

/// Determines if two links are equivalent for the purpose of dependency resolution.
///
/// This function implements the matching logic used when looking up dependencies
/// in the lock file. Different link types use different matching strategies:
///
/// - **GitHub links**: Use "loose" equality that ignores specific commit hashes
///   and compares repository and path only
/// - **Other links**: Use exact equality
fn links_equivalent(x: &Link, y: &Link) -> bool {
    match (x, y) {
        (Link::Github(x), Link::Github(y)) => x.loose_eq(y),
        (x, y) => x == y,
    }
}
