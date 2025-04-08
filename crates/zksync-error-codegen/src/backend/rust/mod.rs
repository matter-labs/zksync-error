pub mod config;
pub mod error;
pub mod files;
pub mod util;

pub use config::Config as RustBackendConfig;
use error::GenerationError;
use proc_macro2::TokenStream;
use util::codegen::ident;
use util::codegen::sanitize;
use util::codegen::type_ident;
use zksync_error_model::inner::ComponentMetadata;
use zksync_error_model::inner::DomainMetadata;
use zksync_error_model::unpacked::UnpackedModel;

use zksync_error_model::error::ModelValidationError;
use zksync_error_model::inner::ErrorDescription;
use zksync_error_model::inner::Model;

use super::Backend;
use super::File;

pub struct RustBackend {
    config: RustBackendConfig,
    model: Model,
    all_domains: Vec<TokenStream>,
    all_domain_codes: Vec<TokenStream>,
}

impl Backend for RustBackend {
    type Config = RustBackendConfig;
    type GenerationError = GenerationError;
    fn get_name() -> &'static str {
        "rust"
    }

    fn get_language_name() -> &'static str {
        "rust"
    }

    fn generate(&mut self) -> Result<Vec<File>, Self::GenerationError> {
        Ok([
            Some(self.generate_file_error_definitions()?),
            Some(self.generate_file_error_domains()?),
            Some(self.generate_file_documentation()?),
            Some(self.generate_file_error_mod()?),
            Some(self.generate_file_identifier()?),
            Some(self.generate_file_kind()?),
            Some(self.generate_file_lib()?),
            Some(self.generate_file_packed()?),
            Some(self.generate_file_serialized()?),
            Some(self.generate_file_untyped()?),
            self.generate_file_cargo()?,
            Some(File {
                relative_path: "resources/error-model-dump.json".into(),
                content: {
                    let unpacked: UnpackedModel =
                        zksync_error_model::unpacked::flatten(&self.model);
                    let user_facing_model: zksync_error_description::ErrorHierarchy =
                        unpacked.into();
                    serde_json::to_string_pretty(&user_facing_model.wrap())?
                },
            }),
        ]
        .into_iter()
        .flatten()
        .collect())
    }

    fn new(config: Self::Config, model: &Model) -> Self {
        let all_domains: Vec<_> = model
            .domains
            .values()
            .map(|domain| Self::domain_ident(&domain.meta))
            .collect();

        let all_domain_codes: Vec<_> = model
            .domains
            .values()
            .map(|domain| Self::domain_code_ident(&domain.meta))
            .collect();

        Self {
            config,
            model: model.clone(),
            all_domains,
            all_domain_codes,
        }
    }
}

impl RustBackend {
    fn format_with_preamble(contents: impl ToString) -> Result<String, rustfmt_wrapper::Error> {
        let preamble = RustBackendConfig::PREAMBLE;
        rustfmt_wrapper::rustfmt(format!(
            r#"
//
// {preamble}
//
{}
"#,
            contents.to_string()
        ))
    }

    fn get_rust_type(&self, name: &str) -> Result<String, GenerationError> {
        let typ = self.model.get_type(Self::get_language_name(), name)?;
        Ok(typ.expression.clone())
    }

    fn component_type_name(component: &ComponentMetadata) -> Result<String, GenerationError> {
        let name = component.bindings.get(Self::get_language_name()).ok_or(
            ModelValidationError::UnmappedName(component.identifier.name.clone()),
        )?;

        Ok(name.to_string())
    }
    fn component_code_type_name(component: &ComponentMetadata) -> Result<String, GenerationError> {
        let name = component.bindings.get(Self::get_language_name()).ok_or(
            ModelValidationError::UnmappedName(component.identifier.name.clone()),
        )?;

        Ok(format!("{name}Code"))
    }

    fn domain_type_name(domain: &DomainMetadata) -> Result<String, GenerationError> {
        let name = domain.bindings.get(Self::get_language_name()).ok_or(
            ModelValidationError::UnmappedName(domain.identifier.name.clone()),
        )?;

        Ok(name.to_string())
    }

    fn domain_code_type_name(domain: &DomainMetadata) -> Result<String, GenerationError> {
        let name = domain.bindings.get(Self::get_language_name()).ok_or(
            ModelValidationError::UnmappedName(domain.identifier.name.clone()),
        )?;

        Ok(format!("{name}Code"))
    }
    fn domain_code_ident(domain: &DomainMetadata) -> TokenStream {
        type_ident(&Self::domain_code_type_name(domain).expect("Internal error"))
    }
    fn domain_ident(domain: &DomainMetadata) -> TokenStream {
        type_ident(&Self::domain_type_name(domain).expect("Internal error"))
    }
    fn component_code_ident(component: &ComponentMetadata) -> TokenStream {
        type_ident(&Self::component_code_type_name(component).expect("Internal error"))
    }
    fn component_ident(component: &ComponentMetadata) -> TokenStream {
        type_ident(&Self::component_type_name(component).expect("Internal error"))
    }
    fn component_error_alias_ident(component: &ComponentMetadata) -> TokenStream {
        type_ident(&format!(
            "{}Error",
            &Self::component_type_name(component).expect("Internal error")
        ))
    }

    fn component_result_alias_ident(component: &ComponentMetadata) -> TokenStream {
        type_ident(&format!(
            "{}Result",
            &Self::component_type_name(component).expect("Internal error")
        ))
    }
    fn error_variant_name(error: &ErrorDescription) -> Result<String, GenerationError> {
        let name = error
            .bindings
            .get(Self::get_language_name())
            .ok_or(ModelValidationError::UnmappedName(error.name.clone()))?;
        Ok(sanitize(&name.expression))
    }

    fn error_ident(error: &ErrorDescription) -> TokenStream {
        ident(&Self::error_variant_name(error).expect("Internal error"))
    }
}
