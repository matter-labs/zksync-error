use quote::quote;
use std::path::PathBuf;

use crate::backend::File;
use crate::backend::rust::RustBackend;
use crate::backend::rust::error::GenerationError;
use crate::backend::rust::util::codegen::ComponentContext;
use crate::backend::rust::util::codegen::DomainContext;
use crate::backend::rust::util::codegen::map_components;
use crate::backend::rust::util::codegen::map_domains;

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
            let documentation_branches =
                map_domains(&self.model, |DomainContext { domain, .. }| {
                    quote! {
                        ZksyncError::#domain ( error ) => error.get_documentation() ,
                    }
                });

            quote! {
                #[cfg(feature="runtime_documentation")]
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

        let display = {
            let display_branches = map_domains(&self.model, |DomainContext { domain, .. }| {
                quote! {
                    ZksyncError::#domain ( domain_error ) => domain_error.fmt(f),
                }
            });

            quote! {
            impl fmt::Display for ZksyncError {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                                    match self {
                                        #( #display_branches )*
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
                    #[derive(IntoStaticStr, Clone, Debug, EnumDiscriminants, Eq, PartialEq)]
                    #[strum_discriminants(derive(FromRepr))]
                    #[cfg_attr(feature = "use_serde", derive(serde::Serialize))]
                    #[cfg_attr(feature = "use_serde", derive(serde::Deserialize))]
                    #[strum_discriminants(name(#domain_code))]
                    #[cfg_attr(feature = "use_serde", strum_discriminants(derive(serde::Serialize, serde::Deserialize)))]
                    #[strum_discriminants(vis(pub))]
                    pub enum #domain {
                        #( #components( #components ),)*
                    }

                    impl #domain {
                        pub fn get_name(&self) -> &'static str {
                            self.into()
                        }
                    }
                    #(
                        impl ICustomError<ZksyncError, ZksyncError> for #components {
                            fn to_unified(&self) -> ZksyncError {
                                 #domain :: #components (self.clone()).to_unified()
                            }
                        }

                        impl From<#components> for #domain {
                            fn from(val: #components) -> Self {
                                #domain::#components(val)
                            }
                        }
                    )*


                    impl ICustomError<ZksyncError, ZksyncError> for #domain {
                        fn to_unified(&self) -> ZksyncError {
                            ZksyncError::#domain(self.clone())
                        }
                    }

                    impl From<#domain> for ZksyncError {
                        fn from(value: #domain) -> Self {
                            value.to_unified()
                        }
                    }

                    #[cfg(feature="runtime_documentation")]
                    impl crate::documentation::Documented for #domain {
                        type Documentation = &'static zksync_error_description::ErrorDocumentation;
                        fn get_documentation(
                            &self,
                        ) -> Result<Option<Self::Documentation>, crate::documentation::DocumentationError> {
                            match self {
                                #( #domain :: #components(error) => error.get_documentation(),) *
                            }
                        }
                    }

                    impl fmt::Display for #domain {
                        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                            match self {
                                #( #domain :: #components(component) => component.fmt(f), ) *
                            }
                        }
                    }
                    impl core::error::Error for #domain {}

                }
            },
        );

        let contents = quote! {

            #![allow(non_camel_case_types)]

            use core::fmt;

            use crate::error::ICustomError;
            use crate::error::IUnifiedError;
            use crate::kind::Kind;
            use strum_macros::IntoStaticStr;
            use strum_macros::EnumDiscriminants;
            use strum_macros::FromRepr;
            #(
                use crate::error::definitions:: #component_idents ;
                use crate::error::definitions:: #component_code_idents ;
            )*

            #[repr(u32)]
            #[derive(IntoStaticStr, Clone, Debug, EnumDiscriminants, Eq, PartialEq)]
            #[cfg_attr(feature = "use_serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum ZksyncError {
                #( #all_domains( #all_domains ),)*
            }

            #documented

            #display

            #impl_zksync_error

            impl IUnifiedError<ZksyncError> for ZksyncError {}
            impl core::error::Error for ZksyncError {}


            #( #component_definitions )*
        };

        Ok(File {
            relative_path: PathBuf::from("src/error/domains.rs"),
            content: Self::format_with_preamble(contents)?,
        })
    }
}
