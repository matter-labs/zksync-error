use quote::quote;
use std::path::PathBuf;

use crate::backend::File;
use crate::backend::rust::RustBackend;
use crate::backend::rust::error::GenerationError;

impl RustBackend {
    pub fn generate_file_error_mod(&mut self) -> Result<File, GenerationError> {
        let domains = &self.all_domains;

        let impl_custom_error_message_writer = {
            let match_tokens =
                self.model.domains.values().flat_map(|domain_description| {
                    let domain = Self::domain_ident(&domain_description.meta);

                    domain_description.components.values().map( move |component_description|  {
                        let component = Self::component_ident(&component_description.meta);
                        quote! {
                            ZksyncError:: #domain ( #domain :: #component (error)) => error.write_message(writer)
                        }
                    }
                    )

                });

            quote! {
                fn write_message<W:core::fmt::Write>(&self, writer:&mut W) -> core::fmt::Result {
                    match self {
                        #( #match_tokens , )*
                    }
                }

            }
        };

        let result = quote! {


            pub(crate) mod definitions;
            pub(crate) mod domains;


            use core::error::Error;
            use crate::identifier::Identifier;
            use crate::error::domains::ZksyncError;

            #( use crate::error::domains:: #domains ; )*

            pub trait IError<ContainedType>: Error
            where
                ContainedType: Clone,
            {
                fn get_identifier(&self) -> Identifier;
                fn write_message<W:core::fmt::Write>(&self, writer:&mut W) -> core::fmt::Result ;
                fn get_data(&self) -> ContainedType;
            }

            #[cfg(not(feature = "use_serde"))]
            pub trait IUnifiedError<ContainedType>:
            IError<ContainedType>
            where
                ContainedType: Clone,
            {
            }
            #[cfg(feature = "use_serde")]
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


            pub trait CustomErrorMessageWriter {
                fn write_message<W: core::fmt::Write>(&self, writer: &mut W) -> core::fmt::Result;
            }
            #[cfg(feature = "std")]
            pub trait CustomErrorMessage {
                fn get_message(&self) -> String;
            }
            #[cfg(feature = "std")]
            impl <T> CustomErrorMessage for T
            where T: CustomErrorMessageWriter {
                fn get_message(&self) -> String {
                    let mut s = String::new();
                    self.write_message(&mut s);
                    s
                }
            }

            impl CustomErrorMessageWriter for ZksyncError {
                #impl_custom_error_message_writer
            }
            #[cfg(feature = "std")]
            pub trait NamedError {
                fn get_error_name(&self) -> &'static str;
            }

            impl IError<ZksyncError> for ZksyncError {
                fn get_identifier(&self) -> Identifier {
                    Identifier {
                        kind: self.get_kind(),
                        code: self.get_code(),
                    }
                }
                fn write_message<W:core::fmt::Write>(&self, writer:&mut W) -> core::fmt::Result {
                    <Self as CustomErrorMessageWriter>::write_message(self, writer)
                }

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
