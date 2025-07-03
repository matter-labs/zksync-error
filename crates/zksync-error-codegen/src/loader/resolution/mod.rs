pub mod context;
pub mod error;
pub mod overrides;

use std::path::PathBuf;

use context::ResolutionContext;
use error::ResolutionError;
use zksync_error_model::link::{
    Link,
    github::{CommitHash, GithubLink, ReferenceType},
};

use super::dependency_lock::{DependencyEntry, DependencyLock};

pub struct ResolutionResult {
    pub actual: Link,
    pub resolved: ResolvedLink,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ResolvedLink {
    LocalPath(PathBuf),
    EmbeddedPath(PathBuf),
    GithubLink(GithubLink),
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
        ResolutionContext::LockOrPopulate { lock, overrides } => {
            if let Some(overridden) = overrides.apply(query_link) {
                Ok(ResolutionResult {
                    actual: overridden.clone(),
                    resolved: resolve_no_lock(query_link),
                })
            } else {
                let actual = query_link.clone();
                if DependencyLock::should_lock(query_link) {
                    let resolved = resolve_with_lock(query_link, lock)?;
                    lock.add_dependency(DependencyEntry {
                        link: actual.clone(),
                        resolved: resolved.clone(),
                    });
                    Ok(ResolutionResult { actual, resolved })
                } else {
                    Ok(ResolutionResult {
                        actual: query_link.clone(),
                        resolved: resolve_no_lock(query_link),
                    })
                }
            }
        }
        ResolutionContext::LockOnly { lock } => {
            let actual = query_link.clone();
            let resolved = if DependencyLock::should_lock(query_link) {
                resolve_lock_only(query_link, lock)?
            } else {
                resolve_no_lock(query_link)
            };

            Ok(ResolutionResult { actual, resolved })
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
        Link::Github(github_link) => ResolvedLink::GithubLink(github_link.clone()),
    }
}

/// If a github link points to a branch AND is absent from the lockfile, then we
/// resolve it to a link  are resolved to the head commit of the respective
/// branch
fn resolve_with_lock(
    query_link: &Link,
    lock: &DependencyLock,
) -> Result<ResolvedLink, ResolutionError> {
    if let Some(DependencyEntry { resolved, .. }) = lock.get_dependency(query_link) {
        Ok(resolved.clone())
    } else {
        match query_link {
            Link::Github(gh_link) => resolve_github_link_to_exact_commit(gh_link),
            other => Ok(resolve_no_lock(other)),
        }
    }
}

fn resolve_github_link_to_exact_commit(
    gh_link: &GithubLink,
) -> Result<ResolvedLink, ResolutionError> {
    match gh_link.reference {
        ReferenceType::Branch { .. } => {
            let commit = get_head_commit_sha(gh_link)?;
            eprintln!("Resolving {gh_link}: head commit is {commit}");
            Ok(ResolvedLink::GithubLink(GithubLink {
                reference: ReferenceType::Commit { commit },
                ..gh_link.clone()
            }))
        }
        ReferenceType::Commit { .. } => Ok(ResolvedLink::GithubLink(gh_link.clone())),
    }
}

fn resolve_lock_only(
    query_link: &Link,
    lock: &DependencyLock,
) -> Result<ResolvedLink, ResolutionError> {
    if let Some(DependencyEntry { resolved, .. }) = lock.get_dependency(query_link) {
        Ok(resolved.clone())
    } else {
        Err(ResolutionError::MissingDependencyInLockFile {
            link: query_link.clone(),
        })
    }
}

fn get_head_commit_sha(link: &GithubLink) -> Result<CommitHash, ResolutionError> {
    let client = reqwest::blocking::Client::new();

    match &link.reference {
        ReferenceType::Commit { commit } => Ok(commit.clone()),
        ReferenceType::Branch { branch } => {
            let url = format!(
                "https://api.github.com/repos/{repo}/commits/{branch}",
                repo = link.repo
            );

            let resp: serde_json::Value = client
                .get(&url)
                .header("User-Agent", "github-sha-fetcher")
                .send()?
                .error_for_status()?
                .json()?;

            if let Some(sha) = resp.get("sha").and_then(|s| s.as_str()) {
                Ok(CommitHash(sha.to_string()))
            } else {
                Err(ResolutionError::MissingShaField { link: link.clone() })
            }
        }
    }
}
