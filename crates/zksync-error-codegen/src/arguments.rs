//! Argument structures for code generation.
//!
//! This module defines the argument structures used to configure the
//! error code generation process, including output backends, resolution
//! modes, and general generation parameters.

use std::path::PathBuf;

/// Configuration for a single backend output.
///
/// Each backend output represents a target format for the generated code,
/// such as Rust code or MDBook documentation.
pub struct BackendOutput {
    /// The directory path where the generated output should be written
    pub output_path: PathBuf,
    /// The backend type to use for generation
    pub backend: Backend,
    /// Backend-specific arguments as key-value pairs
    pub arguments: Vec<(String, String)>,
}

/// Dependency resolution mode configuration.
///
/// This enum defines the different modes for resolving dependencies during
/// code generation, each with different characteristics regarding lock files
/// and reproducibility.
pub enum ResolutionMode {
    /// No lock file mode.
    ///
    /// Dependencies are resolved without using or creating lock files.
    /// This mode is useful for development builds where reproducibility
    /// is not required.
    NoLock {
        /// Link override mappings for dependency resolution
        override_links: Vec<(String, String)>,
    },
}
/// Complete set of arguments for code generation.
///
/// This structure contains all the configuration needed to perform
/// error code generation, from input sources to output targets.
pub struct GenerationArguments {
    /// Enable verbose logging during generation
    pub verbose: bool,
    /// List of input link strings to process
    pub input_links: Vec<String>,
    /// Dependency resolution mode configuration
    pub mode: ResolutionMode,
    /// List of backend outputs to generate
    pub outputs: Vec<BackendOutput>,
}

#[derive(Clone, Debug)]
pub enum Backend {
    Rust,
    Mdbook,
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Backend::Rust => "rust",
            Backend::Mdbook => "doc-mdbook",
        })
    }
}
