use std::fs;
use std::path::Path;

use reqwest;

use zksync_error_model::link::Link;

use crate::loader::error::LoadError;
use crate::loader::resolution::{ResolvedLink, resolve};

use super::resolution::{ResolutionContext, ResolutionResult};

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

fn from_network(url: &str) -> Result<String, reqwest::Error> {
    eprintln!("Fetching file from network: {url}");
    let response = reqwest::blocking::get(url)?;
    let content = response.text()?;
    Ok(content)
}

fn from_embedded(path: &Path) -> Result<String, LoadError> {
    if let Some(path) = super::EMBEDDED_DESCRIPTIONS_DIR
        .get_file(path)
        .map(|f| f.path())
    {
        from_fs(&path.to_owned())
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
}

pub fn load_text(link: &Link, context: &ResolutionContext) -> Result<LoadResult, LoadError> {
    let ResolutionResult { actual, resolved } = resolve(link, context)?;
    let text = match resolved {
        ResolvedLink::DescriptionFile(description_file) => {
            from_fs(&description_file.absolute_path)?
        }
        ResolvedLink::LocalPath(path) => from_fs(&path)?,
        ResolvedLink::Url(url) => from_network(&url)?,
        ResolvedLink::Immediate(immediate) => immediate,
        ResolvedLink::EmbeddedPath(path_buf) => from_embedded(&path_buf)?,
    };

    Ok(LoadResult { text, actual })
}
