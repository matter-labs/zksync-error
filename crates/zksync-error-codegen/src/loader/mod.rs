use cargo::get_resolution_context;
use error::FileFormatError;
use error::LinkError;
use error::LoadError;
use error::ResolutionError;
use link::Link;

use std::path::PathBuf;

use crate::description::Collection;

pub mod builder;
pub mod cargo;
pub mod error;
pub mod fetch;
pub mod link;

#[derive(Clone, Debug)]
pub struct DescriptionFile {
    pub package: String,
    pub absolute_path: PathBuf,
}

#[derive(Clone, Debug, Default)]
pub struct ResolutionContext {
    pub files: Vec<DescriptionFile>,
}

impl ResolutionContext {
    pub fn find_package(&self, package: &str) -> Option<&DescriptionFile> {
        self.files.iter().find(|df| &df.package == package)
    }
}

pub enum ResolvedLink {
    DescriptionFile(DescriptionFile),
    LocalPath(PathBuf),
    Url(String),
}

impl Link {
    fn resolve(query_link: &Link, context: &ResolutionContext) -> Result<ResolvedLink, LinkError> {
        match query_link {
            link @ Link::PackageLink { .. } => {
                if let Some(df) = context.files.iter().find(|file| Link::matches(link, file)) {
                    Ok(ResolvedLink::DescriptionFile(df.clone()))
                } else {
                    Err(LinkError::FailedResolution(
                        ResolutionError::CargoLinkResolutionError {
                            link: link.clone(),
                            context: context.clone(),
                        },
                    ))
                }
            }
            Link::FileLink { path } => Ok(ResolvedLink::LocalPath(path.into())),
            Link::URL { url } => Ok(ResolvedLink::Url(url.to_owned())),
        }
    }
}

pub fn load(link: &Link) -> Result<Collection, LoadError> {
    let context = get_resolution_context();
    match Link::resolve(link, &context)? {
        ResolvedLink::DescriptionFile(description_file) => {
            let contents = std::fs::read_to_string(&description_file.absolute_path)?;
            load_serialized(&contents)
        }
        ResolvedLink::LocalPath(path) => {
            let contents = fetch::from_fs(&path)?;
            load_serialized(&contents)
        }
        ResolvedLink::Url(url) => {
            let contents = fetch::from_network(&url)?;
            load_serialized(&contents)
        }
    }
}

pub fn load_serialized(contents: &str) -> Result<Collection, LoadError> {
    serde_json::from_str::<crate::description::Collection>(contents).map_err(|error| {
        LoadError::FileFormatError(FileFormatError::ParseError(
            contents.to_owned(),
            Box::new(error),
        ))
    })
}
