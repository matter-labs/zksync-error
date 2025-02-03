use quote::quote;
use std::path::PathBuf;

use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_identifier(&mut self) -> Result<File, GenerationError> {
        let domain_codes = &self.all_domain_codes;
        let domains = &self.all_domains;

        let imports = quote! {

            #(use crate::error::domains:: #domain_codes ;)*

            use crate::error::NamedError;
            use crate::kind::DomainCode;
            use crate::kind::Kind;
        };
        let def_structured_error_code = quote! {

            #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
            pub struct StructuredErrorCode {
                pub domain_code: u32,
                pub component_code: u32,
                pub error_code: u32,
            }


        };

        let impl_structured_error_code = quote! {
                        impl StructuredErrorCode {
                pub fn encode(&self) -> u32 {
                    self.domain_code * 10000 + self.component_code * 1000 + self.error_code
                }

                pub fn decode(raw_code: u32) -> Self {
                    let error_code = raw_code % 1000;
                    let component_code = (raw_code / 1000) % 10;
                    let domain_code = (raw_code / 10000) % 10;
                    StructuredErrorCode {
                        domain_code,
                        component_code,
                        error_code,
                    }
                }
            }
        };
        let def_identifier = quote! {
            #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
            pub struct Identifier {
                pub kind: Kind,
                pub code: u32,
            }

        };
        let impl_identifier = quote! {

                impl Identifier {
                    pub fn new(kind: Kind, code: u32) -> Self {
                        Self { kind, code }
                    }

                    pub fn encode(&self) -> u32 {
                        let domain_code: u32 = self.kind.domain_code();
                        let component_code: u32 = self.kind.component_code();
                        domain_code * 10000 + component_code * 1000 + self.code
                    }

                    pub fn decode(code: StructuredErrorCode) -> Option<Self> {
                        let StructuredErrorCode { domain_code, component_code, error_code } = code;
                        let domain = DomainCode::from_repr(domain_code)?;
                        let kind: Kind = match domain {

                            #(
                                DomainCode:: #domains => Kind:: #domains ( #domain_codes :: from_repr( component_code )? ) ,
                            )*
                        };
                        Some(Identifier { kind, code: error_code })
                    }
                }
        };

        let trait_identifying = quote! {
                pub trait Identifying {
                    fn get_identifier_repr(&self)-> String;
                }

                impl Identifying for Identifier {
                    fn get_identifier_repr(&self) -> String {
                        format!("[{}-{}]", self.kind.get_identifier_repr(), self.code)
                    }
                }
        };

        let impl_identifying_for_kind = {
            let match_tokens = self.model.domains.values().flat_map(|domain_description| {
                let domain_code = Self::domain_code_ident(&domain_description.meta);
                let domain = Self::domain_ident(&domain_description.meta);
                let domain_contribution = &domain_description.meta.identifier;

                domain_description
                    .components
                    .values()
                    .map(move |component_description| {
                        let component = Self::component_ident(&component_description.meta);
                        let component_contribution = &component_description.meta.identifier;
                        let prefix = format!("{domain_contribution}-{component_contribution}");
                        quote! {
                            Kind :: #domain ( #domain_code :: #component ) =>
                                #prefix
                        }
                    })
            });

            quote! {
                impl Identifying for Kind {
                    fn get_identifier_repr(&self) -> String {
                        match self {
                            #( #match_tokens , )*
                        }.into()
                    }
                }
            }
        };

        let impl_named_error = {
            let match_tokens =
                self.model.domains.values().flat_map(|domain_description| {
                    let domain_code = Self::domain_code_ident(&domain_description.meta);
                    let domain = Self::domain_ident(&domain_description.meta);

                    domain_description.components.values().map( move |component_description|  {
                        let component_code = Self::component_code_ident(&component_description.meta);
                        let component = Self::component_ident(&component_description.meta);

                        quote! {
                            Kind :: #domain ( #domain_code :: #component ) =>
                                crate::error::definitions:: #component_code ::from_repr(self.code).expect("Internal error").get_error_name()
                        }
                    }
                    )

                });

            quote! {
                impl NamedError for Identifier {
                    fn get_error_name(&self) -> String {
                        match self.kind {
                            #( #match_tokens ),*
                        }
                    }
                }
            }
        };
        let impl_documented = quote! {
                impl crate::documentation::Documented for Identifier {
                    type Documentation = &'static zksync_error_description::ErrorDocumentation;
                    fn get_documentation(&self) -> Result<Option<Self::Documentation>, crate::documentation::DocumentationError> {
                        use crate::documentation::model;

                        let repr = &self.get_identifier_repr();
                        match model.errors.get(repr) {
                            Some(metadata) => Ok(metadata.documentation.as_ref()),
                            None => Err(crate::documentation::DocumentationError::IncompleteModel(format!("Can not fetch description for error {repr}.")))
                        }
                    }
                }
        };

        let result = quote! {
                #imports

                #def_structured_error_code

                #impl_structured_error_code

                #def_identifier

                #impl_identifier

                #trait_identifying

                #impl_identifying_for_kind

                #impl_named_error

                #impl_documented

        };

        Ok(File {
            content: Self::format_with_preamble(&result)?,
            relative_path: PathBuf::from("src/identifier.rs"),
        })
    }
}
