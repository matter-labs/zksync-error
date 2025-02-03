use quote::quote;
use std::path::PathBuf;

use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_error_mod(&mut self) -> Result<File, GenerationError> {
        let domains = &self.all_domains;

        let impl_ierror_getmessage = {
            let match_tokens =
                self.model.domains.values().flat_map(|domain_description| {
                    let domain = Self::domain_ident(&domain_description.meta);

                    domain_description.components.values().map( move |component_description|  {
                        let component = Self::component_ident(&component_description.meta);
                        quote! {
                            ZksyncError:: #domain ( #domain :: #component (error)) => error.get_message()
                        }
                    }
                    )

                });

            quote! {
                fn get_message(&self) -> String {
                    match self {
                        #( #match_tokens , )*
                    }
                }

            }
        };

        let result = quote! {


            pub mod definitions;
            pub mod domains;

            use std::error::Error;
            use crate::identifier::Identifier;
            use crate::error::domains::ZksyncError;

            #( use crate::error::domains:: #domains ; )*

            pub trait IError<ContainedType>: Error
            where
                ContainedType: Clone,
            {
                fn get_identifier(&self) -> Identifier;
                fn get_message(&self) -> String;
                fn get_data(&self) -> ContainedType;
            }

            pub trait IUnifiedError<ContainedType>:
            serde::Serialize + for<'de> serde::Deserialize<'de> + IError<ContainedType>
            where
                ContainedType: Clone,
            {
            }

            pub trait ICustomError<U, C>
            where
                U: IUnifiedError<C>,
                C: Clone,
            {
                fn to_unified(&self) -> U;
            }

            pub trait CustomErrorMessage {
                fn get_message(&self) -> String;
            }

            pub trait NamedError {
                fn get_error_name(&self) -> String;
            }

            impl IError<ZksyncError> for ZksyncError {
                fn get_identifier(&self) -> Identifier {
                    Identifier {
                        kind: self.get_kind(),
                        code: self.get_code(),
                    }
                }

                #impl_ierror_getmessage

                fn get_data(&self) -> ZksyncError {
                    self.clone()
                }
            }
        };

        Ok(File {
            content: Self::format_with_preamble(result)?,
            relative_path: PathBuf::from("src/error/mod.rs"),
        })
    }
}
