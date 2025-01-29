use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct File {
    pub relative_path: PathBuf,
    pub content: String,
}
