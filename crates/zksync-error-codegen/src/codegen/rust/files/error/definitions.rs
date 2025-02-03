use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;
use zksync_error_model::inner::ComponentDescription;

use crate::codegen::rust::config::Config;
use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::util::codegen::doc_tokens;
use crate::codegen::rust::util::codegen::ident;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;
use zksync_error_model::inner::ErrorDescription;
use zksync_error_model::inner::ErrorDocumentation;
use zksync_error_model::inner::FieldDescription;

fn error_documentation(description: &ErrorDescription) -> TokenStream {
    if let Some(ErrorDocumentation {
        description,
        summary,
        ..
    }) = description.documentation.as_ref()
    {
        let summary = summary.clone().unwrap_or_default();
        let result = if description.is_empty() {
            summary
        } else {
            format!("# Summary \n{summary}\n\n# Description\n{description}\n")
        };
        let result_trimmed = result.trim();
        if result_trimmed.is_empty() {
            quote! {}
        } else {
            doc_tokens(result_trimmed)
        }
    } else {
        quote! {}
    }
}

fn component_doc(component: &ComponentDescription) -> TokenStream {
    doc_tokens(&format!(
        "{}

Domain: {}",
        component.meta.description, component.meta.domain.name,
    ))
}

impl RustBackend {
    fn error_variant(&self, error: &ErrorDescription) -> Result<TokenStream, GenerationError> {
        let ErrorDescription { code, fields, .. } = error;
        let mut field_tokens = Vec::new();
        for FieldDescription { name, r#type } in fields {
            let name = ident(name);
            let typ = ident(&self.get_rust_type(r#type)?);
            field_tokens.push(quote! { #name : #typ  });
        }
        let error_name = RustBackend::error_ident(error);
        let doc = error_documentation(error);
        let field_tokens_if_nonempty = if fields.is_empty() {
            quote! {}
        } else {
            quote! { {  #( #field_tokens , )* } }
        };
        Ok(quote! { #doc
                     #error_name #field_tokens_if_nonempty = # code
        })
    }

    pub fn generate_file_error_definitions(
        &mut self,
        config: &Config,
    ) -> Result<File, GenerationError> {
        let definitions = self.model.components().map(|component| -> TokenStream {


            let component_code = RustBackend::component_code_ident(&component.meta);
            let error_variants = component.errors.iter().flat_map(|component| self.error_variant(component));
            let component_name = RustBackend::component_ident(&component.meta);

            let component_doc = component_doc(component);
            let from_anyhow =
                config.use_anyhow.then_some(
                    quote! {
                        impl From<anyhow::Error> for #component_name {
                            fn from(value: anyhow::Error) -> Self {
                                let message = format!("{value:#?}");
                                #component_name::GenericError { message }
                            }
                        }
                    });

            let impl_custom_error_message = {

                let branch_patterns = component.errors.iter().map(|error| {
                    let error_name = RustBackend::error_ident(error);
                    let field_tokens = if error.fields.is_empty() {
                        quote! { }
                    }
                    else {
                        let pattern_fields = error.fields.iter().map( | field | ident(&field.name));
                        quote! { {  #( #pattern_fields, )* } }
                    };
                    quote! { #component_name :: #error_name #field_tokens }
                });

                let messages = component.errors.iter().map(|error| { format!("{} {}", error.get_identifier(), error.message) } );
                quote! {
                    impl CustomErrorMessage for #component_name {
                        fn get_message(&self) -> String {
                            match self {
                                #( #branch_patterns => { format! ( #messages ) } , )*
                            }
                        }
                    }
                }

            };
            quote! {

                #component_doc
                #[repr(u32)]
                #[derive(AsRefStr, Clone, Debug, Eq, EnumDiscriminants, PartialEq, serde::Serialize, serde::Deserialize)]
                #[strum_discriminants(name(#component_code))]
                #[strum_discriminants(vis(pub))]
                #[strum_discriminants(derive(AsRefStr, FromRepr))]
                #[non_exhaustive]
                pub enum #component_name {

                    #( #error_variants , )*
                }

                impl std::error::Error for #component_name {}

                impl NamedError for #component_name {
                    fn get_error_name(&self) -> String {
                        self.as_ref().to_owned()
                    }
                }
                impl NamedError for #component_code {
                    fn get_error_name(&self) -> String {
                        self.as_ref().to_owned()
                    }
                }

                impl From<#component_name> for crate::ZksyncError {
                    fn from(val: #component_name) -> Self {
                        val.to_unified()
                    }
                }
                impl std::fmt::Display for #component_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_fmt(format_args!("{self:?}"))
                    }
                }
                impl Documented for #component_name {
                    type Documentation = &'static zksync_error_description::ErrorDocumentation;

                    fn get_documentation(&self) -> Result<Option<Self::Documentation>, crate::documentation::DocumentationError> {
                        self.to_unified().get_identifier().get_documentation()
                    }
                }
                #from_anyhow


                impl From<#component_name> for crate::packed::PackedError<crate::error::domains::ZksyncError> {
                    fn from(value: #component_name) -> Self {
                        crate::packed::pack(value)
                    }
                }

                impl From<#component_name> for crate::serialized::SerializedError {
                    fn from(value: #component_name) -> Self {
                        let packed = crate::packed::pack(value);
                        crate::serialized::serialize(packed).expect("Internal serialization error.")
                    }
                }

                #impl_custom_error_message
            }

        });

        let contents = quote! {
            #![allow(unused)]
            #![allow(non_camel_case_types)]

            use crate::documentation::Documented;
            use crate::error::CustomErrorMessage;
            use crate::error::NamedError;
            use crate::error::ICustomError as _;
            use crate::error::IError as _;
            use strum_macros::AsRefStr;
            use strum_macros::EnumDiscriminants;
            use strum_macros::FromRepr;

            #( #definitions )*
        };

        Ok(File {
            content: Self::format_with_preamble(contents)?,
            relative_path: PathBuf::from("src/error/definitions.rs"),
        })
    }
}
