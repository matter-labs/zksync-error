use quote::quote;
use std::path::PathBuf;

use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_documentation(&mut self) -> Result<File, GenerationError> {
        let contents = quote! {
            use lazy_static::lazy_static;
            use zksync_error_description::ErrorHierarchy;

            lazy_static! {
                pub static ref model : ErrorHierarchy = get_model();
            }


            fn get_model() -> ErrorHierarchy {
                zksync_error_description::ErrorHierarchy::from_str(include_str!("../resources/error-model-dump.json"))
            }

            #[derive(Debug)]
            pub enum DocumentationError {
                IncompleteModel(String),

            }

            impl std::fmt::Display for DocumentationError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{self:#?}"))
                }
            }
            impl std::error::Error for DocumentationError {}

            pub trait Documented {
                type Documentation;
                fn get_documentation(&self) -> Result<Option<Self::Documentation>, DocumentationError>;
            }
        };
        Ok(File {
            content: Self::format_with_preamble(contents)?,
            relative_path: PathBuf::from("src/documentation.rs"),
        })
    }
}
