//!
//! Backends for the code generator exposed through CLI.
//!

use clap::Parser;

///
/// Backends for the code generator exposed through CLI.
///
#[derive(Clone, Debug, Parser)]
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
impl std::str::FromStr for Backend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" => Ok(Backend::Rust),
            "doc-mdbook" => Ok(Backend::Mdbook),
            _ => Err("Unrecognized backend".into()),
        }
    }
}

impl From<Backend> for zksync_error_codegen::arguments::Backend {
    fn from(value: Backend) -> Self {
        match value {
            Backend::Rust => Self::Rust,
            Backend::Mdbook => Self::Mdbook,
        }
    }
}
