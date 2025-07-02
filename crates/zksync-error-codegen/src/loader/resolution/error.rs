use zksync_error_model::link::{Link, github::GithubLink};

#[derive(Debug, thiserror::Error)]
pub enum ResolutionError {
    #[error("Missing dependency in lock file: {link}")]
    MissingDependencyInLockFile { link: Link },
    #[error("Can't fetch the SHA hash of the latest commit for the link {link} ")]
    MissingShaField { link: GithubLink },
    #[error("Can't access Github: {inner}")]
    GithubAccessError {
        #[from]
        inner: reqwest::Error,
    },
}
