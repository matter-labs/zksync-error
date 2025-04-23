pub mod error;

use std::path::PathBuf;

use error::ResolutionError;
use zksync_error_model::link::Link;

use super::cargo::{CollectionFile, link_matches};

#[derive(Clone, Debug, Default)]
pub struct ResolutionContext {
    pub files: Vec<CollectionFile>,
}

impl ResolutionContext {
    pub fn find_package(&self, package: &str) -> Option<&CollectionFile> {
        self.files.iter().find(|df| df.package == package)
    }
}

pub enum ResolvedLink {
    DescriptionFile(CollectionFile),
    LocalPath(PathBuf),
    Url(String),
    Immediate(String),
}

pub fn resolve(
    query_link: &Link,
    context: &ResolutionContext,
) -> Result<ResolvedLink, ResolutionError> {
    match query_link {
        link @ Link::PackageLink { .. } => {
            if let Some(df) = context.files.iter().find(|file| link_matches(link, file)) {
                Ok(ResolvedLink::DescriptionFile(df.clone()))
            } else {
                Err(ResolutionError::CargoLinkResolutionError {
                    link: link.clone(),
                    context: context.clone(),
                })
            }
        }
        Link::FileLink { path } => Ok(ResolvedLink::LocalPath(path.into())),
        Link::URL { url } => Ok(ResolvedLink::Url(url.to_owned())),
        Link::DefaultLink => Ok(ResolvedLink::Immediate(
            super::ZKSYNC_ROOT_CONTENTS.to_owned(),
        )),
    }
}
