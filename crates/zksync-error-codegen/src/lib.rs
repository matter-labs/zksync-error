pub mod arguments;
pub mod backend;
pub mod description;
pub mod error;
pub mod loader;

use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;

use arguments::Backend;
use arguments::GenerationArguments;
use backend::IBackendConfig as _;
use error::ProgramError;
use loader::builder::build_model;
use loader::link::Link;
use zksync_error_model::inner::Model;

use crate::backend::file::File;
use crate::backend::mdbook::MDBookBackend;
use crate::backend::rust::RustBackend;
use crate::backend::Backend as CodegenBackend;

pub fn default_load_and_generate(root_link: &str, input_links: Vec<&str>) {
    if let Err(e) = load_and_generate(GenerationArguments {
        verbose: true,
        root_link: root_link.to_owned(),
        outputs: vec![arguments::BackendOutput {
            output_path: "../zksync_error".into(),
            backend: Backend::Rust,
            arguments: vec![],
        }],
        input_links: input_links.into_iter().map(Into::into).collect(),
    }) {
        eprintln!("{e:#?}")
    };
}

fn generate<Backend>(
    backend_args: impl Iterator<Item = (String, String)>,
    model: &Model,
) -> Result<Vec<File>, ProgramError>
where
    Backend: CodegenBackend,
{
    let config = Backend::Config::parse_arguments(backend_args).map_err(|error| {
        ProgramError::BackendError {
            backend_name: Backend::get_name().to_string(),
            inner: Box::new(error),
        }
    })?;
    Backend::new(config, model)
        .generate()
        .map_err(|error| ProgramError::BackendError {
            backend_name: Backend::get_name().to_string(),
            inner: Box::new(error),
        })
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

    for arguments::BackendOutput {
        output_path,
        backend,
        arguments: backend_arguments,
    } in outputs
    {
        if *verbose {
            eprintln!("Selected backend: {backend:?}, \nGenerating files...");
        }

        let result: Vec<File> = match backend {
            Backend::Rust => generate::<RustBackend>(backend_arguments.iter().cloned(), &model)?,
            Backend::Mdbook => {
                generate::<MDBookBackend>(backend_arguments.iter().cloned(), &model)?
            }
        };

        if *verbose {
            eprintln!("Generation successful. Files: ");
            for file in &result {
                eprintln!("- {}", file.relative_path.to_str().unwrap());
            }
            eprintln!("Writing files to disk...");
        }

        create_files_in_result_directory(output_path, result)?;
        if *verbose {
            eprintln!("Writing successful.");
        }
    }
    Ok(())
}

fn create_files_in_result_directory(result_dir: &PathBuf, files: Vec<File>) -> std::io::Result<()> {
    let result_dir = Path::new(result_dir);

    if !result_dir.exists() {
        std::fs::create_dir(result_dir)?;
    }

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
