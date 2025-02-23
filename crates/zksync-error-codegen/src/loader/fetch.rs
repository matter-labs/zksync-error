use std::fs;
use std::path::PathBuf;

use reqwest;

use zksync_error_model::link::Link;

use crate::loader::cargo::get_resolution_context;
use crate::loader::error::LoadError;
use crate::loader::resolution::{resolve, ResolvedLink};

fn from_fs(path: &PathBuf) -> Result<String, LoadError> {
    eprintln!(
        "Reading local file: {}",
        path.to_str().expect("Incorrect path")
    );
    fs::read_to_string(path).map_err(|inner| LoadError::IOError {
        path: path.clone(),
        inner,
    })
}

fn from_network(url: &str) -> Result<String, reqwest::Error> {
    eprintln!("Fetching file from network: {url}");
    let response = reqwest::blocking::get(url)?;
    let content = response.text()?;
    Ok(content)
}

pub fn load_text(link: &Link) -> Result<String, LoadError> {
    let context = get_resolution_context();
    Ok(match resolve(link, &context)? {
        ResolvedLink::DescriptionFile(description_file) => {
            from_fs(&description_file.absolute_path)?
        }
        ResolvedLink::LocalPath(path) => from_fs(&path)?,
        ResolvedLink::Url(url) => from_network(&url)?,
        ResolvedLink::Immediate(immediate) => immediate,
    })
}
