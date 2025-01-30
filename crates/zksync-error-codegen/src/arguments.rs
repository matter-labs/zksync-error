use std::path::PathBuf;
use vector_map::VecMap;

pub struct GenerationArguments {
    pub verbose: bool,
    pub root_link: String,
    pub input_links: Vec<String>,
    pub outputs: Vec<(PathBuf, Backend, VecMap<String, String>)>,
}

#[derive(Clone, Debug)]
pub enum Backend {
    DocHtml,
    Rust,
    Mdbook,
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Backend::DocHtml => "doc-html",
            Backend::Rust => "rust",
            Backend::Mdbook => "doc-mdbook",
        })
    }
}
