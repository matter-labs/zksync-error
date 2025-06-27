use github::GithubLink;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod github;

/// Represents different types of links to JSON files.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub enum Link {
    /// A link to a bundled resource embedded within the application.
    /// The path is relative to the embedded resources root.
    Bundled { path: String },

    /// A link to a file on the local filesystem.
    /// The path can be absolute or relative.
    FileLink { path: String },

    /// A link to a GitHub resource.
    /// See `GithubLink` for more details on the structure.
    Github(GithubLink),

    /// A generic URL link.
    /// Typically used for HTTP/HTTPS resources.
    URL { url: String },
}

impl Display for Link {
    /// Formats the link for display purposes.
    ///
    /// - URLs and file links are displayed as-is
    /// - Bundled links are wrapped in angle brackets with "embedded:" prefix
    /// - GitHub links use their own Display implementation
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Link::URL { url } => f.write_str(url),
            Link::FileLink { path } => f.write_str(path),
            Link::Bundled { path } => write!(f, "<embedded: {path}>"),
            Link::Github(github_link) => github_link.fmt(f),
        }
    }
}
