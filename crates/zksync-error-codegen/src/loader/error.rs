use std::fmt::Display;

use super::builder::error::ModelBuildingError;
use super::link::Link;
use super::resolution::error::ResolutionError;

#[derive(Debug)]
pub enum LinkError {
    InvalidLinkFormat(String),
    FailedResolution(ResolutionError),
}

impl std::fmt::Display for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkError::InvalidLinkFormat(link) =>
                f.write_fmt(format_args!("Link `{link}` has an invalid format. Expected `{}://<crate_name>{}<filename-with-extension>`.", Link::CARGO_FORMAT_PREFIX, Link::PACKAGE_SEPARATOR)),
            LinkError::FailedResolution(r) => r.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum FileFormatError {
    ExpectedFullGotDomain { source: Link },
    ExpectedFullGotComponent { source: Link },
    ExpectedFullGotErrors { source: Link },
    ParseError(String, Box<dyn std::error::Error>),
}

impl std::fmt::Display for FileFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileFormatError::ExpectedFullGotErrors { source }  =>
                f.write_fmt(format_args!("File `{source}` contains just an array of errors, but a master error database should describe at least one domain and one component.")),
            FileFormatError::ExpectedFullGotDomain{ source }  =>
                f.write_fmt(format_args!("File `{source}` contains just an error domain description, but a master error database should describe at least one domain and one component.")),
            FileFormatError::ExpectedFullGotComponent{source} =>
                f.write_fmt(format_args!("File `{source}` contains just an error component description, but a master error database should describe at least one domain and one component.")),
            FileFormatError::ParseError(path, error) => f.write_fmt(format_args!("Error parsing file `{path}`: {error}")),
        }
    }
}

#[derive(Debug)]
pub enum LoadError {
    IOError(std::io::Error),
    NetworkError(reqwest::Error),
    FileFormatError(FileFormatError),
    LinkError(LinkError),
    ResolutionError(ResolutionError),
    MissingFileError(String),
    ModelBuildingError(Box<ModelBuildingError>),
}

impl From<ModelBuildingError> for LoadError {
    fn from(v: ModelBuildingError) -> Self {
        Self::ModelBuildingError(Box::new(v))
    }
}

impl From<ResolutionError> for LoadError {
    fn from(v: ResolutionError) -> Self {
        Self::ResolutionError(v)
    }
}

impl From<LinkError> for LoadError {
    fn from(v: LinkError) -> Self {
        Self::LinkError(v)
    }
}

impl From<FileFormatError> for LoadError {
    fn from(v: FileFormatError) -> Self {
        Self::FileFormatError(v)
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:#?}"))
    }
}
impl From<reqwest::Error> for LoadError {
    fn from(v: reqwest::Error) -> Self {
        Self::NetworkError(v)
    }
}

impl From<std::io::Error> for LoadError {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}
