use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;

use crate::backend::rust::error::GenerationError;
use crate::backend::rust::util::codegen::ident;
use crate::backend::rust::RustBackend;
use crate::backend::File;

impl RustBackend {
    pub fn generate_file_lib(&mut self) -> Result<File, GenerationError> {
        let imports = quote! {

            #![allow(unused)]

            pub mod documentation;
            pub(crate) mod error;
            pub use error::IError;
            pub use error::IUnifiedError;
            pub use error::ICustomError;
            pub use error::CustomErrorMessage;
            pub use error::NamedError;

            pub(crate) mod identifier;
            pub use identifier::StructuredErrorCode;
            pub use identifier::Identifier;
            pub use identifier::Identifying;
            pub(crate) mod kind;
            pub use kind::Kind;

            pub mod packed;
            pub mod serialized;
            pub mod untyped;

            pub use crate::error::domains::ZksyncError;

        };

        let interface_modules = self.model.domains.values().map( |domain| ->TokenStream{
            let outer_module = ident(&domain.meta.identifier);

            let domain_name = RustBackend::domain_ident(&domain.meta);
            let domain_error_name = ident(&format!("{domain_name}Error"));
            let domain_code_name = RustBackend::domain_code_ident(&domain.meta);
            let component_modules = domain.components.values().flat_map( |component| ->TokenStream {


                let inner_module = ident(&component.meta.identifier);
                let enum_name = Self::component_ident(&component.meta);
                let enum_code_name = Self::component_code_ident(&component.meta);
                let alias = Self::component_error_alias_ident(&component.meta);
                let errors = component.errors.iter().map(Self::error_ident);
                let macro_name = ident(&format!("{outer_module}_{inner_module}_generic_error"));

                quote! {
                    pub mod #inner_module {
                        pub use crate::error::definitions:: #enum_name  as #alias ;
                        pub use crate::error::definitions:: #enum_code_name as ErrorCode;
                        #(
                            pub use crate::error::definitions:: #enum_name :: #errors ;
                        )*

                        #[macro_export]
                        macro_rules! #macro_name {
                            ($($arg:tt)*) => {
                                zksync_error::#outer_module::#inner_module:: #alias::GenericError { message: format!($($arg)*) }
                            };
                        }
                        pub use crate:: #macro_name as generic_error;

                        pub fn to_generic<T: std::fmt::Display>(err: T) -> #alias {
                            GenericError {
                                message: err.to_string(),
                            }
                        }
                        pub fn to_domain<T: std::fmt::Display>(err: T) -> super::#domain_error_name {
                            super::#domain_error_name::#enum_name( GenericError {
                                message: err.to_string(),
                            })
                        }
                    }
                }
            });

            quote!{
                pub mod #outer_module {

                   pub use crate::error::domains::#domain_name as #domain_error_name;
                   pub use crate::error::domains::#domain_code_name;
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
