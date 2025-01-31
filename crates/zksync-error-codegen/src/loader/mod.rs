use cargo::get_resolution_context;
use error::FileFormatError;
use error::LoadError;
use link::Link;
use resolution::resolve;
use resolution::ResolvedLink;

use std::path::PathBuf;

use crate::description::Collection;

pub mod builder;
pub mod cargo;
pub mod error;
pub mod link;
pub mod resolution;

#[derive(Clone, Debug)]
pub struct CollectionFile {
    pub package: String,
    pub absolute_path: PathBuf,
}

pub fn load(link: &Link) -> Result<Collection, LoadError> {
    let context = get_resolution_context();
    let contents = match resolve(link, &context)? {
        ResolvedLink::DescriptionFile(description_file) => {
            fetch::from_fs(&description_file.absolute_path)?
        }
        ResolvedLink::LocalPath(path) => fetch::from_fs(&path)?,
        ResolvedLink::Url(url) => fetch::from_network(&url)?,
    };

    load_serialized(&contents)
}

pub fn load_serialized(contents: &str) -> Result<Collection, LoadError> {
    serde_json::from_str::<crate::description::Collection>(contents).map_err(|error| {
        LoadError::FileFormatError(FileFormatError::ParseError {
            contents: contents.to_owned(),
            inner: Box::new(error),
        })
    })
}

mod fetch {
    use reqwest;
    use std::fs;
    use std::path::PathBuf;

    pub fn from_fs(path: &PathBuf) -> std::io::Result<String> {
        eprintln!(
            "Trying to read local file: {}",
            path.to_str().expect("Incorrect path")
        );
        fs::read_to_string(path)
    }

    pub fn from_network(url: &str) -> Result<String, reqwest::Error> {
        eprintln!("Trying to fetch file from network: {url}");
        let response = reqwest::blocking::get(url)?;
        let content = response.text()?;
        Ok(content)
    }
}
