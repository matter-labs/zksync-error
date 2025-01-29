use std::path::PathBuf;

use super::{error::LinkError, DescriptionFile};

#[derive(Clone, Debug)]
pub enum Link {
    PackageLink { package: String, filename: String },
    FileLink { path: String },
    URL { url: String },
}
impl Link {
    /// Part before "://"
    pub const CARGO_FORMAT_PREFIX: &str = "cargo";
    pub const FILE_FORMAT_PREFIX: &str = "file";
    pub const NETWORK_FORMAT_PREFIXES: [&str; 2] = ["https", "http"];
    pub const PACKAGE_SEPARATOR: &str = "@@";

    pub fn parse(link: impl Into<String>) -> Result<Link, LinkError> {
        let string: String = link.into();

        match string.split_once("://") {
            Some((Link::CARGO_FORMAT_PREFIX, path)) => {
                match path.split_once(Link::PACKAGE_SEPARATOR) {
                    Some((package, filename)) => Ok(Link::PackageLink {
                        package: package.to_owned(),
                        filename: filename.to_owned(),
                    }),
                    None => Err(LinkError::InvalidLinkFormat(string)),
                }
            }
            Some((Link::FILE_FORMAT_PREFIX, path)) => Ok(Link::FileLink {
                path: path.to_owned(),
            }),
            Some((prefix, _)) if Link::NETWORK_FORMAT_PREFIXES.contains(&prefix) => Ok(Link::URL {
                url: string.to_string(),
            }),
            None => Ok(Link::FileLink { path: string }),
            Some(_) => Err(LinkError::InvalidLinkFormat(string)),
        }
    }
    pub fn matches(link: &Link, file: &DescriptionFile) -> bool {
        if let Link::PackageLink { package, filename } = link {
            let DescriptionFile {
                package: candidate_package,
                absolute_path,
            } = file;

            if package != candidate_package {
                return false;
            };
            let pathbuf = PathBuf::from(absolute_path);
            let stripped_filename = pathbuf
                .file_name()
                .unwrap_or_else(|| panic!("Error accessing file `{absolute_path:?}`."));

            stripped_filename.to_str().is_some_and(|s| s == filename)
        } else {
            false
        }
    }
}

impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Link::PackageLink { package, filename } => f.write_fmt(format_args!(
                "{}://{package}{}{filename}",
                Link::CARGO_FORMAT_PREFIX,
                Link::PACKAGE_SEPARATOR
            )),
            Link::URL { url } => f.write_str(url),
            Link::FileLink { path } => f.write_str(path),
        }
    }
}
