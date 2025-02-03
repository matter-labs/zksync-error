pub mod arguments;
pub mod codegen;
pub mod description;
pub mod error;
pub mod loader;

use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;

use arguments::Backend;
use arguments::GenerationArguments;
use error::ProgramError;
use loader::builder::build_model;
use loader::link::Link;

use crate::codegen::file::File;
use crate::codegen::mdbook::config::MDBookBackendConfig;
use crate::codegen::mdbook::MDBookBackend;
use crate::codegen::rust::RustBackend;
use crate::codegen::rust::RustBackendConfig;
use crate::codegen::Backend as _;

pub fn default_load_and_generate(root_link: &str, input_links: Vec<&str>) {
    if let Err(e) = load_and_generate(GenerationArguments {
        verbose: true,
        root_link: root_link.to_owned(),
        outputs: vec![("../zksync_error".into(), Backend::Rust, vec![])],
        input_links: input_links.into_iter().map(Into::into).collect(),
    }) {
        eprintln!("{e:#?}")
    };
}

pub fn load_and_generate(arguments: GenerationArguments) -> Result<(), ProgramError> {
    let GenerationArguments {
        verbose,
        root_link,
        outputs,
        input_links,
    } = &arguments;
    if *verbose {
        eprintln!("Reading config from \"{root_link}\"");
    }

    let additions: Result<Vec<_>, _> = input_links.iter().map(Link::parse).collect();
    let model = build_model(&Link::parse(root_link)?, &additions?, *verbose)?;

    for (output_directory, backend_type, backend_arguments) in outputs {
        let backend_arguments = vector_map::VecMap::from_iter(backend_arguments.iter().cloned());
        if *verbose {
            eprintln!("Selected backend: {backend_type:?}, \nGenerating files...");
        }

        let result = match backend_type {
            arguments::Backend::Rust => {
                let mut backend = RustBackend::new(&model);
                backend.generate(&RustBackendConfig {
                    use_anyhow: std::str::FromStr::from_str(
                        backend_arguments
                            .get(&String::from("use_anyhow"))
                            .unwrap_or(&String::from("false")),
                    )
                    .unwrap(),
                })?
            }
            arguments::Backend::Mdbook => {
                let mut backend = MDBookBackend::new(&model);
                backend.generate(&MDBookBackendConfig)?
            }
        };

        if *verbose {
            eprintln!("Generation successful. Files: ");
            for file in &result {
                eprintln!("- {}", file.relative_path.to_str().unwrap());
            }
            eprintln!("Writing files to disk...");
        }

        create_files_in_result_directory(output_directory, result)?;
        if *verbose {
            eprintln!("Writing successful.");
        }
    }
    Ok(())
}

fn create_files_in_result_directory(result_dir: &PathBuf, files: Vec<File>) -> std::io::Result<()> {
    let result_dir = Path::new(result_dir);

    if result_dir.exists() {
        std::fs::remove_dir_all(result_dir)?;
    }

    std::fs::create_dir(result_dir)?;

    for file in files {
        let path = result_dir.join(file.relative_path);

        if let Some(parent_dir) = path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let mut output_file = std::fs::File::create(&path)?;
        output_file.write_all(file.content.as_bytes())?;
    }

    Ok(())
}
