use quote::quote;
use std::path::PathBuf;

use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_kind(&mut self) -> Result<File, GenerationError> {
        let domains = &self.all_domains;
        let domain_codes = &self.all_domain_codes;
        let codes = self.model.domains.values().map(|d| d.meta.code);

        let contents = quote! {

            use strum_macros::EnumDiscriminants;
            use strum_macros::FromRepr;

            use crate::error::domains::AnvilCode;
            use crate::error::domains::CompilerCode;
            use crate::error::domains::CoreCode;
            use crate::error::domains::FoundryCode;
            use crate::error::domains::HardhatCode;

            #[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
            #[strum_discriminants(name(DomainCode))]
            #[strum_discriminants(derive(FromRepr))]
            #[strum_discriminants(vis(pub))]
            #[repr(u32)]
            pub enum Kind {
                #( #domains ( #domain_codes ) = #codes ,)*
            }

            impl Kind {
                pub fn domain_code(&self) -> u32 {
                    let domain: DomainCode = self.clone().into();
                    domain as u32
                }
                pub fn component_code(&self) -> u32 {
                    match self {
                        #( Kind:: #domains (component) => component.clone() as u32, )*
                    }
                }
            }

        };

        Ok(File {
            content: Self::format_with_preamble(contents)?,
            relative_path: PathBuf::from("src/kind.rs"),
        })
    }
}
