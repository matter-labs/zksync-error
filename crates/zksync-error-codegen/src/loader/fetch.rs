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
