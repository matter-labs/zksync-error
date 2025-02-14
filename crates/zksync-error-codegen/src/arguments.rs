use std::path::PathBuf;

pub struct BackendOutput {
    pub output_path: PathBuf,
    pub backend: Backend,
    pub arguments: Vec<(String, String)>,
}

pub struct GenerationArguments {
    pub verbose: bool,
    pub root_link: String,
    pub input_links: Vec<String>,
    pub outputs: Vec<BackendOutput>,
}

#[derive(Clone, Debug)]
pub enum Backend {
    Rust,
    Mdbook,
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Backend::Rust => "rust",
            Backend::Mdbook => "doc-mdbook",
        })
    }
}
