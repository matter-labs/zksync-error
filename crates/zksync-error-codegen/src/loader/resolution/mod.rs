pub mod error;
pub mod overrides;

use std::path::PathBuf;

use error::ResolutionError;
use overrides::Remapping;
use zksync_error_model::link::Link;

use super::cargo::{CollectionFile, link_matches};

#[derive(Clone, Debug)]
pub struct ResolutionContext {
    pub files: Vec<CollectionFile>,
    pub overrides: Remapping,
}

impl ResolutionContext {
    pub fn find_package(&self, package: &str) -> Option<&CollectionFile> {
        self.files.iter().find(|df| df.package == package)
    }
}

pub struct ResolutionResult {
    pub actual: Link,
    pub resolved: ResolvedLink,
}
pub enum ResolvedLink {
    DescriptionFile(CollectionFile),
    LocalPath(PathBuf),
    EmbeddedPath(PathBuf),
    Url(String),
    Immediate(String),
}

pub fn resolve(
    query_link: &Link,
    context: &ResolutionContext,
) -> Result<ResolutionResult, ResolutionError> {
    match context.overrides.map.get(query_link).cloned() {
        Some(overridden) => {
            //TODO: eventually a stack to keep track of the path
            eprintln!("Overriding {query_link} with {overridden}...");
            resolve(&overridden, context)
        }
        None => {
            let resolved = match query_link {
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
                Link::Embedded { path } => Ok(ResolvedLink::EmbeddedPath(
                    format!(
                        "{manifest}/../../descriptions/{path}",
                        manifest = env!("CARGO_MANIFEST_DIR")
                    )
                    .into(),
                )),
            }?;
            Ok(ResolutionResult {
                actual: query_link.clone(),
                resolved,
            })
        }
    }
}
