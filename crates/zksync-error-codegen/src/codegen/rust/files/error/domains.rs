use quote::quote;
use std::path::PathBuf;

use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::util::codegen::map_components;
use crate::codegen::rust::util::codegen::map_domains;
use crate::codegen::rust::util::codegen::ComponentContext;
use crate::codegen::rust::util::codegen::DomainContext;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_error_domains(&mut self) -> Result<File, GenerationError> {
        let all_domains = &self.all_domains;

        let component_idents = self
            .model
            .components()
            .map(|component| RustBackend::component_ident(&component.meta));
        let component_code_idents = self
            .model
            .components()
            .map(|component| RustBackend::component_code_ident(&component.meta));

        let documented = {
            let documentation_branches = map_components(
                &self.model,
                |ComponentContext {
                     domain, component, ..
                 }| {
                    quote! {
                        ZksyncError::#domain ( #domain :: #component (error)) => error.get_documentation() ,
                    }
                },
            );

            quote! {
                impl crate::documentation::Documented for ZksyncError {
                    type Documentation = &'static zksync_error_description::ErrorDocumentation;

                    fn get_documentation(&self) -> Result<Option<Self::Documentation>, crate::documentation::DocumentationError> {
                        match self {
                            #( #documentation_branches )*
                        }
                    }
                }
            }
        };

        let impl_zksync_error = {
            let get_kind = {
                let branches = map_components(
                    &self.model,
                    |ComponentContext {
                         domain,
                         domain_code,
                         component,
                         ..
                     }| {
                        quote! {
                            ZksyncError::#domain( #domain :: #component(_)) => { Kind::#domain (#domain_code :: #component) }
                        }
                    },
                );
                quote! {
                    pub fn get_kind(&self) -> crate::kind::Kind {
                        match self {
                            #( #branches , )*
                        }
                    }
                }
            };
            let get_code = {
                let branches = map_components(
                    &self.model,
                    |ComponentContext {
                         domain,
                         component,
                         component_code,
                         ..
                     }| {
                        quote! {
                            ZksyncError:: #domain (#domain :: #component(error)) => { Into::< #component_code >::into(error) as u32 },
                        }
                    },
                );
                quote! {
                    pub fn get_code(&self) -> u32 {
                        match self {
                            #( #branches )*
                        }
                    }
                }
            };
            quote! {
                impl ZksyncError {

                    #get_kind

                    #get_code
                }
            }
        };

        let component_definitions = map_domains(
            &self.model,
            |DomainContext {
                 domain,
                 domain_code,
                 components,
                 ..
             }| {
                quote! {

                    #[repr(u32)]
                    #[derive(AsRefStr, Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
                    #[strum_discriminants(name(#domain_code))]
                    #[strum_discriminants(derive(serde::Serialize, serde::Deserialize, FromRepr))]
                    #[strum_discriminants(vis(pub))]
                    pub enum #domain {
                        #( #components( #components ),)*
                    }

                    impl #domain {
                        pub fn get_name(&self) -> &str {
                            self.as_ref()
                        }
                    }
                    #(
                        impl ICustomError<ZksyncError, ZksyncError> for #components {
                            fn to_unified(&self) -> ZksyncError {
                                ZksyncError::#domain( #domain :: #components (self.clone()))
                            }
                        }
                    )*
                }
            },
        );

        let contents = quote! {

            #![allow(non_camel_case_types)]
            use crate::error::ICustomError;
            use crate::error::IUnifiedError;
            use crate::kind::Kind;
            use strum_macros::AsRefStr;
            use strum_macros::EnumDiscriminants;
            use strum_macros::FromRepr;
            #(
                use crate::error::definitions:: #component_idents ;
                use crate::error::definitions:: #component_code_idents ;
            )*

            #[repr(u32)]
            #[derive(AsRefStr, Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
            pub enum ZksyncError {
                #( #all_domains( #all_domains ),)*
            }

            #documented

            #impl_zksync_error

            impl std::fmt::Display for ZksyncError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{:#?}", self))
                }
            }
            impl IUnifiedError<ZksyncError> for ZksyncError {}
            impl std::error::Error for ZksyncError {}


            #( #component_definitions )*
        };

        Ok(File {
            relative_path: PathBuf::from("src/error/domains.rs"),
            content: Self::format_with_preamble(contents)?,
        })
    }
}
