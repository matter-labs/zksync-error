use quote::quote;
use std::path::PathBuf;

use crate::backend::File;
use crate::backend::rust::RustBackend;
use crate::backend::rust::error::GenerationError;

impl RustBackend {
    pub fn generate_file_kind(&mut self) -> Result<File, GenerationError> {
        let domains = &self.all_domains;
        let domain_codes = &self.all_domain_codes;
        let codes = self.model.domains.values().map(|d| d.meta.identifier.code);

        let contents = quote! {

            use strum_macros::EnumDiscriminants;
            use strum_macros::FromRepr;

            #(use crate::error::domains:: #domain_codes ;)*

            #[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq)]
            #[cfg_attr(feature = "use_serde", derive(serde::Serialize, serde::Deserialize))]
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
                        #( Kind:: #domains (component) => *component as u32, )*
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
