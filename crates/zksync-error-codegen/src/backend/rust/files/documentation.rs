use quote::quote;
use std::path::PathBuf;

use crate::backend::File;
use crate::backend::rust::RustBackend;
use crate::backend::rust::error::GenerationError;

impl RustBackend {
    pub fn generate_file_documentation(&mut self) -> Result<File, GenerationError> {
        let contents = quote! {
            use core::fmt;
            use lazy_static::lazy_static;
            use zksync_error_description::ErrorHierarchy;

            lazy_static! {
                pub static ref model : ErrorHierarchy = get_model();
            }


            fn get_model() -> ErrorHierarchy {
                zksync_error_description::ErrorHierarchy::deserialize(include_str!("../resources/error-model-dump.json"))
            }

            #[derive(Debug)]
            pub enum DocumentationError {
                IncompleteModel(String),

            }

            impl fmt::Display for DocumentationError {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.write_fmt(format_args!("{self:#?}"))
                }
            }
            impl core::error::Error for DocumentationError {}

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
