pub mod error;
pub mod overrides;

use std::path::PathBuf;

use error::ResolutionError;
use overrides::Remapping;
use zksync_error_model::link::Link;

#[derive(Clone, Debug)]
pub struct ResolutionContext {
    pub overrides: Remapping,
}

pub struct ResolutionResult {
    pub actual: Link,
    pub resolved: ResolvedLink,
}
pub enum ResolvedLink {
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
                Link::FileLink { path } => Ok(ResolvedLink::LocalPath(path.into())),
                Link::URL { url } => Ok(ResolvedLink::Url(url.to_owned())),
                Link::Bundled { path } => Ok(ResolvedLink::EmbeddedPath(
                    format!(
                        "{manifest}/../../descriptions/{path}",
                        manifest = env!("CARGO_MANIFEST_DIR")
                    )
                    .into(),
                )),
                Link::Github(github_link) => Ok(ResolvedLink::Url(github_link.to_url())),
            }?;
            Ok(ResolutionResult {
                actual: query_link.clone(),
                resolved,
            })
        }
    }
}
