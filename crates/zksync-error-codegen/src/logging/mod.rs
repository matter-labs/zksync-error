use std::{fmt::Display, path::PathBuf};

const LOGGING_DIR: &'static str = "zksync-error-logs";

pub fn log_file<T: Display>(filename: &str, elem: T) {
    let path: PathBuf = PathBuf::from(LOGGING_DIR).join(filename);
    let _ = std::fs::write(path, elem.to_string());
}

pub fn log<T: Display>(elem: T) {
    eprintln!("{elem}")
}
