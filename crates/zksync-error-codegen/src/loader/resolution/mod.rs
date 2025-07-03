pub mod context;
pub mod error;
pub mod overrides;

use std::path::PathBuf;

use context::ResolutionContext;
use error::ResolutionError;
use zksync_error_model::link::Link;

pub struct ResolutionResult {
    pub actual: Link,
    pub resolved: ResolvedLink,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ResolvedLink {
    LocalPath(PathBuf),
    EmbeddedPath(PathBuf),
    Url(String),
}

pub fn resolve(
    query_link: &Link,
    context: &mut ResolutionContext,
) -> Result<ResolutionResult, ResolutionError> {
    match context {
        ResolutionContext::NoLock { overrides } => {
            let with_override = overrides.apply(query_link).unwrap_or(query_link);
            Ok(ResolutionResult {
                actual: with_override.clone(),
                resolved: resolve_no_lock(with_override),
            })
        }
    }
}

fn resolve_no_lock(query_link: &Link) -> ResolvedLink {
    match query_link {
        Link::FileLink { path } => ResolvedLink::LocalPath(path.into()),
        Link::URL { url } => ResolvedLink::Url(url.to_owned()),
        Link::Bundled { path } => ResolvedLink::EmbeddedPath(
            format!(
                "{manifest}/../../descriptions/{path}",
                manifest = env!("CARGO_MANIFEST_DIR")
            )
            .into(),
        ),
        Link::Github(github_link) => ResolvedLink::Url(github_link.to_url()),
    }
}
