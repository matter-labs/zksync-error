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
        ResolvedLink::Immediate(immediate) => immediate,
    };

    load_serialized(&contents)
}

fn pretty_print_fragment(text: &str, line: usize, column: usize) -> String {
    let half_window = 3;
    let first_line = (line - half_window).max(0);
    let last_line_excl = (line + half_window).min(text.lines().count());

    let mut result = String::with_capacity(1024);
    for (text_line, line_no) in text
        .lines()
        .skip(first_line)
        .take(last_line_excl - first_line)
        .zip(first_line + 1..)
    {
        result.push_str(&format!("{line_no:6}{text_line}\n"));
        if line_no == line {
            for _ in 0..column + 6 {
                result.push(' ');
            }
            result.push_str("^\n");
        }
    }
    result
}
pub fn load_serialized(contents: &str) -> Result<Collection, LoadError> {
    serde_json_path_to_error::from_str::<crate::description::Collection>(contents).map_err(
        |error| {
            let inner = error.inner();
            LoadError::FileFormatError(FileFormatError::ParseError {
                contents: pretty_print_fragment(contents, inner.line(), inner.column()),
                inner: Box::new(error),
            })
        },
    )
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

pub(crate) static ZKSYNC_ROOT_CONTENTS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../zksync-root.json"
));
