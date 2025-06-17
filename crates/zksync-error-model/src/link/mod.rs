use const_format::concatcp;

use error::LinkError;

pub mod error;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Link {
    Bundled { path: String },
    FileLink { path: String },
    URL { url: String },
}

impl Link {
    /// Part before "://"
    pub const FILE_FORMAT_PREFIX: &str = "file";
    pub const ZKSYNC_DESCRIPTIONS_LOCATION: &str = "descriptions/";
    pub const EMBEDDED_FORMAT_PREFIX: &str = "zksync-error";
    pub const DEFAULT_ROOT_FILE_NAME_NO_EXTENSION: &str = "zksync-root";
    pub const DEFAULT_ROOT_FILE_PATH: &str = concatcp!(
        Link::ZKSYNC_DESCRIPTIONS_LOCATION,
        Link::DEFAULT_ROOT_FILE_NAME_NO_EXTENSION,
        ".json"
    );
    pub const NETWORK_FORMAT_PREFIXES: [&str; 2] = ["https", "http"];
    pub const PACKAGE_SEPARATOR: &str = "@@";

    pub fn parse(link: &str) -> Result<Link, LinkError> {
        let string: String = link.into();

        match string.split_once("://") {
            Some((Link::FILE_FORMAT_PREFIX, path)) => Ok(Link::FileLink {
                path: path.to_owned(),
            }),
            Some((Link::EMBEDDED_FORMAT_PREFIX, path)) => Ok(Link::Bundled {
                path: path.to_owned(),
            }),
            Some((prefix, _)) if Link::NETWORK_FORMAT_PREFIXES.contains(&prefix) => {
                Ok(Link::URL { url: string })
            }
            None => Ok(Link::FileLink { path: string }),
            Some(_) => Err(LinkError::InvalidLinkFormat(string)),
        }
    }
}

impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Link::URL { url } => f.write_str(url),
            Link::FileLink { path } => f.write_str(path),
            Link::Bundled { path } => write!(f, "<embedded: {path}>"),
        }
    }
}
