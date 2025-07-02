//! Link parsing functionality.
//!
//! This module provides functionality to parse various types of links used in
//! error descriptions, including GitHub links, file links, embedded resources,
//! and HTTP/HTTPS URLs. It handles the conversion from the serialized
//! `TakeFromLink` format to the internal `Link` representation.
//!
//! # Supported Link Formats
//!
//! - **GitHub links**: JSON objects with repo, path, and reference information
//! - **File links**: `file://path/to/file` or bare paths
//! - **Embedded links**: `zksync-error://resource/path`. Gives access to files
//!    that are placed in the directory `/description` in the root of this
//!    repository.
//! - **HTTP/HTTPS URLs**: `http://example.com/resource` or `https://example.com/resource`
//!
use crate::description::TakeFromLink;
use const_format::concatcp;
use zksync_error_model::link::Link;

/// Errors that can occur during link parsing.
#[derive(Debug, derive_more::Display, thiserror::Error)]
pub enum LinkError {
    /// The provided link string has an invalid or unsupported format.
    #[display("Link `{_0}` has an invalid format.")]
    InvalidLinkFormat(String),
}

/// The protocol prefix used to explicitly denote file links.
/// Example: "file://path/to/resource"
pub const FILE_FORMAT_PREFIX: &str = "file";

/// The base directory path for zksync descriptions.
pub const ZKSYNC_DESCRIPTIONS_LOCATION: &str = "descriptions/";

/// The protocol prefix used for embedded/bundled resources.
/// Example: "zksync-error://resource/path"
pub const EMBEDDED_FORMAT_PREFIX: &str = "zksync-error";

/// The default filename (without extension) for the root description file.
pub const DEFAULT_ROOT_FILE_NAME_NO_EXTENSION: &str = "zksync-root";

/// The complete default path to the root description file.
/// Combines the descriptions location with the default root filename and JSON extension.
pub const DEFAULT_ROOT_FILE_PATH: &str = concatcp!(
    ZKSYNC_DESCRIPTIONS_LOCATION,
    DEFAULT_ROOT_FILE_NAME_NO_EXTENSION,
    ".json"
);

/// Array of protocol prefixes that indicate network-based URLs.
pub const NETWORK_FORMAT_PREFIXES: [&str; 2] = ["https", "http"];

/// Separator used in package identifiers.
pub const PACKAGE_SEPARATOR: &str = "@@";

/// Converts a serialized link from JSON file to the internal link
/// representation used by the system.
pub fn parse(link: &TakeFromLink) -> Result<Link, LinkError> {
    match link {
        TakeFromLink::GithubLink(github_link) => Ok(Link::Github(github_link.clone())),
        TakeFromLink::OrdinaryLink(string) => match string.split_once("://") {
            Some((FILE_FORMAT_PREFIX, path)) => Ok(Link::FileLink {
                path: path.to_owned(),
            }),
            Some((EMBEDDED_FORMAT_PREFIX, path)) => Ok(Link::Bundled {
                path: path.to_owned(),
            }),
            Some((prefix, _)) if NETWORK_FORMAT_PREFIXES.contains(&prefix) => Ok(Link::URL {
                url: string.to_owned(),
            }),
            None => Ok(Link::FileLink {
                path: string.to_owned(),
            }),
            Some(_) => Err(LinkError::InvalidLinkFormat(string.to_owned())),
        },
    }
}

/// Parses a link from a string representation.
///
/// The function tries multiple parsing approaches:
/// 1. First, try to parse as JSON (for GitHub links)
/// 2. If that fails, try to parse as a quoted string
/// 3. If both fail, return an error
pub fn parse_str(link: &str) -> Result<Link, LinkError> {
    let take_from_link = serde_json::from_str::<TakeFromLink>(link)
        .or(serde_json::from_str::<TakeFromLink>(&format!("\"{link}\"")))
        .map_err(|_| LinkError::InvalidLinkFormat(link.to_owned()))?;
    parse(&take_from_link)
}

#[cfg(test)]
mod tests {
    use super::*;
    use zksync_error_model::link::{
        Link,
        github::{BranchName, CommitHash, GithubLink, ReferenceType},
    };

    /// Creates a sample GitHub link referencing a branch.
    fn sample_github_link_branch() -> GithubLink {
        GithubLink::new_with_branch(
            "zk/repo".to_string(),
            "path/to/file".to_string(),
            BranchName("dev".to_string()),
        )
    }

    /// Creates a sample GitHub link referencing a specific commit.
    fn sample_github_link_commit() -> GithubLink {
        GithubLink::new_with_commit(
            "zk/repo".to_string(),
            "path/to/file".to_string(),
            CommitHash("deadbeef".to_string()),
        )
    }

    #[test]
    fn test_parse_github_link_branch() {
        let link = TakeFromLink::GithubLink(sample_github_link_branch());
        let parsed = parse(&link).unwrap();
        match parsed {
            Link::Github(gl) => assert_eq!(gl.repo, "zk/repo"),
            _ => panic!("Expected Github link"),
        }
    }

    #[test]
    fn test_parse_github_link_commit() {
        let link = TakeFromLink::GithubLink(sample_github_link_commit());
        let parsed = parse(&link).unwrap();
        match parsed {
            Link::Github(gl) => {
                assert_eq!(gl.repo, "zk/repo");
                assert!(matches!(gl.reference, ReferenceType::Commit { .. }));
            }
            _ => panic!("Expected Github link"),
        }
    }

    #[test]
    fn test_parse_file_link() {
        let link = TakeFromLink::OrdinaryLink("file://path/to/file.json".to_string());
        let parsed = parse(&link).unwrap();
        assert_eq!(
            parsed,
            Link::FileLink {
                path: "path/to/file.json".to_string()
            }
        );
    }

    #[test]
    fn test_parse_embedded_link() {
        let link = TakeFromLink::OrdinaryLink("zksync-error://resource/path".to_string());
        let parsed = parse(&link).unwrap();
        assert_eq!(
            parsed,
            Link::Bundled {
                path: "resource/path".to_string()
            }
        );
    }

    #[test]
    fn test_parse_http_link() {
        let url = "http://example.com/resource.json";
        let link = TakeFromLink::OrdinaryLink(url.to_string());
        let parsed = parse(&link).unwrap();
        assert_eq!(
            parsed,
            Link::URL {
                url: url.to_string()
            }
        );
    }

    #[test]
    fn test_parse_https_link() {
        let url = "https://example.com/resource.json";
        let link = TakeFromLink::OrdinaryLink(url.to_string());
        let parsed = parse(&link).unwrap();
        assert_eq!(
            parsed,
            Link::URL {
                url: url.to_string()
            }
        );
    }

    #[test]
    fn test_parse_bare_file_path() {
        let path = "local/path.json";
        let link = TakeFromLink::OrdinaryLink(path.to_string());
        let parsed = parse(&link).unwrap();
        assert_eq!(
            parsed,
            Link::FileLink {
                path: path.to_string()
            }
        );
    }

    #[test]
    fn test_parse_invalid_prefix() {
        let link = TakeFromLink::OrdinaryLink("invalidprefix://data".to_string());
        let result = parse(&link);
        assert!(matches!(result, Err(LinkError::InvalidLinkFormat(_))));
    }

    #[test]
    fn test_parse_str_valid_github() {
        let json = serde_json::json!({
            "repo": "zk/repo",
            "path": "test/path",
            "branch": "main"
        });
        let json_str = json.to_string();
        let parsed = parse_str(&json_str).unwrap();
        match parsed {
            Link::Github(gl) => assert_eq!(gl.repo, "zk/repo"),
            _ => panic!("Expected Github link"),
        }
    }

    #[test]
    fn test_parse_str_invalid_json() {
        let invalid_json = "{not a valid json}";
        let result = parse_str(invalid_json);
        assert!(matches!(result, Ok(Link::FileLink { .. })));
    }
}
