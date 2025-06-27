pub mod arguments;
pub mod backend;
pub mod description;
pub mod error;
pub mod loader;
pub(crate) mod util;

use arguments::Backend;
use arguments::GenerationArguments;
use backend::IBackendConfig as _;
use description::parsers::link;
use error::ProgramError;
use loader::builder::build_model;
use loader::resolution::overrides::Remapping;
use zksync_error_model::inner::Model;
use zksync_error_model::link::Link;

use crate::backend::Backend as CodegenBackend;
use crate::backend::file::File;
use crate::backend::mdbook::MDBookBackend;
use crate::backend::rust::RustBackend;

pub fn default_load_and_generate(input_links: Vec<&str>) {
    if let Err(e) = load_and_generate(GenerationArguments {
        verbose: true,
        outputs: vec![arguments::BackendOutput {
            output_path: "../zksync_error".into(),
            backend: Backend::Rust,
            arguments: vec![],
        }],
        input_links: input_links.into_iter().map(Into::into).collect(),
        override_links: vec![],
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
        outputs,
        input_links,
        override_links,
    } = arguments;

    let model = {
        let input_links: Result<Vec<Link>, _> = input_links
            .iter()
            .map(|repr| link::parse_str(repr))
            .collect();
        let overrides = Remapping::try_from(&override_links)?;
        build_model(input_links?, overrides, verbose)?
    };

    for arguments::BackendOutput {
        output_path,
        backend,
        arguments: backend_arguments,
    } in outputs
    {
        if verbose {
            eprintln!("Selected backend: {backend:?}, \nGenerating files...");
        }

        let result: Vec<File> = match backend {
            Backend::Rust => generate::<RustBackend>(backend_arguments.iter().cloned(), &model)?,
            Backend::Mdbook => {
                generate::<MDBookBackend>(backend_arguments.iter().cloned(), &model)?
            }
        };

        if verbose {
            eprintln!("Generation successful. Files: ");
            for file in &result {
                eprintln!("- {}", file.relative_path.to_str().unwrap());
            }
            eprintln!("Writing files to disk...");
        }

        crate::util::io::create_files_in_result_directory(&output_path, result)?;
        if verbose {
            eprintln!("Writing successful.");
        }
    }
    Ok(())
}
