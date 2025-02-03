use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;

use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::util::codegen::ident;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_lib(&mut self) -> Result<File, GenerationError> {
        let imports = quote! {

            #![allow(unused)]

            pub mod error;
            pub mod identifier;
            pub mod kind;
            pub mod packed;
            pub mod serialized;
            pub mod untyped;
            pub mod documentation;


            pub use crate::error::domains::ZksyncError;

        };

        let interface_modules = self.model.domains.values().map( |domain| ->TokenStream{
            let outer_module = ident(&domain.meta.identifier);

            let component_modules = domain.components.values().flat_map( |component| ->TokenStream {


                let inner_module = ident(&component.meta.identifier);
                let enum_name = Self::component_ident(&component.meta);
                let alias = Self::component_error_alias_ident(&component.meta);
                let errors = component.errors.iter().map(Self::error_ident);
                let macro_name = ident(&format!("{outer_module}_{inner_module}_generic_error"));

                quote! {
                    pub mod #inner_module {
                        pub use crate::error::definitions:: #enum_name  as #alias ;
                        #(
                            pub use crate::error::definitions:: #enum_name :: #errors ;
                        )*

                        #[macro_export]
                        macro_rules! #macro_name {
                            ($($arg:tt)*) => {
                                zksync_error::error::definitions:: #enum_name ::GenericError { message: format!($($arg)*) }
                            };
                        }
                        pub use crate:: #macro_name as generic_error;
                    }
                }
            });

            quote!{
                pub mod #outer_module {
                   #( #component_modules )*
                }
            }
        }
        );

        let contents = quote! {

            #imports

            #( #interface_modules )*
        };

        Ok(File {
            content: Self::format_with_preamble(contents)?,
            relative_path: PathBuf::from("src/lib.rs"),
        })
    }
}
