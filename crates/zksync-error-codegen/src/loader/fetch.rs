use std::fs;
use std::path::Path;

use reqwest;

use zksync_error_model::link::Link;

use crate::loader::error::LoadError;
use crate::loader::resolution::{ResolvedLink, resolve};

use super::resolution::{ResolutionResult, context::ResolutionContext};

fn from_fs(path: &Path) -> Result<String, LoadError> {
    eprintln!(
        "Reading local file: {}",
        path.to_str().expect("Incorrect path")
    );
    fs::read_to_string(path).map_err(|inner| LoadError::IOError {
        path: path.into(),
        inner,
    })
}

fn from_network(url: &str) -> Result<String, LoadError> {
    eprintln!("Fetching file from network: {url}");
    let response = reqwest::blocking::get(url).map_err(|inner| LoadError::NetworkError {
        url: url.to_string(),
        inner,
    })?;
    let content = response.text().map_err(|inner| LoadError::NetworkError {
        url: url.to_string(),
        inner,
    })?;
    Ok(content)
}

fn from_embedded(path: &Path) -> Result<String, LoadError> {
    if let Some(path) = super::EMBEDDED_DESCRIPTIONS_DIR
        .get_file(path)
        .map(|f| f.path())
    {
        from_fs(path)
    } else {
        fs::read_to_string(path).map_err(|inner| LoadError::IOError {
            path: path.into(),
            inner,
        })
    }
}

pub struct LoadResult {
    pub text: String,
    pub actual: Link,
    pub overridden: bool,
}

pub fn load_text(link: &Link, context: &mut ResolutionContext) -> Result<LoadResult, LoadError> {
    let ResolutionResult {
        actual,
        resolved,
        overridden,
    } = resolve(link, context)?;
    let text = match resolved {
        ResolvedLink::LocalPath(path) => from_fs(&path)?,
        ResolvedLink::Url(url) => from_network(&url)?,
        ResolvedLink::EmbeddedPath(path_buf) => from_embedded(&path_buf)?,
        ResolvedLink::GithubLink(github_link) => from_network(&github_link.to_url())?,
    };

    Ok(LoadResult {
        text,
        actual,
        overridden,
    })
}
