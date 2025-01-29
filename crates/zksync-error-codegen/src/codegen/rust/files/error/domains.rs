use std::path::PathBuf;

use crate::codegen::printer::PrettyPrinter;
use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;
use zksync_error_model::inner::DomainDescription;

impl RustBackend {
    pub fn generate_file_error_domains(&mut self) -> Result<File, GenerationError> {
        let mut gen = PrettyPrinter::default();

        Self::preamble(&mut gen);

        gen.push_str(
            "
#![allow(non_camel_case_types)]
use crate::error::ICustomError;
use crate::error::IUnifiedError;
use crate::kind::Kind;
use strum_macros::AsRefStr;
use strum_macros::EnumDiscriminants;
use strum_macros::FromRepr;
",
        );

        let domains = || self.model.domains.values();
        let components = || {
            self.model
                .domains
                .values()
                .flat_map(|domain| domain.components.values())
        };

        for component in components() {
            let component_error_type = Self::component_type_name(component)?;
            gen.push_line(&format!(
                "use crate::error::definitions::{component_error_type};"
            ));
            gen.push_line(&format!(
                "use crate::error::definitions::{component_error_type}Code;"
            ));
        }

        gen.push_line(
            r#"
#[repr(u32)]
#[derive(AsRefStr, Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ZksyncError {"#,
        );

        gen.indent_more();
        for domain in domains() {
            let constructor = Self::domain_type_name(domain)?;
            let domain_type = &constructor;
            gen.push_line(&format!("{constructor}({domain_type}), "));
        }

        gen.indent_less();

        gen.push_line("}");

        gen.push_line(r#"
impl crate::documentation::Documented for ZksyncError {
    type Documentation = &'static zksync_error_description::ErrorDocumentation;

    fn get_documentation(&self) -> Result<Option<Self::Documentation>, crate::documentation::DocumentationError> {
        match self {
"#);
        gen.indent_more_by(3);

        for domain_description in domains() {
            for component_description in domain_description.components.values() {
                let domain = Self::domain_type_name(domain_description)?;
                let component = Self::component_type_name(component_description)?;
                let _component_code = Self::component_code_type_name(component_description)?;
                let domain_code = Self::domain_code_type_name(domain_description)?;

                gen.push_line(&format!(
                    "ZksyncError::{domain}({domain}::{component}(error)) => error.get_documentation() ,"
                ));
            }
        }

        for _ in 0..3 {
            gen.indent_less();
            gen.push_line("}");
        }

        gen.push_line(
            r#"
impl ZksyncError {

    pub fn get_kind(&self) -> crate::kind::Kind {
        match self {"#,
        );

        gen.indent_more_by(3);

        for domain_description in domains() {
            for component_description in domain_description.components.values() {
                let domain = Self::domain_type_name(domain_description)?;
                let component = Self::component_type_name(component_description)?;
                let _component_code = Self::component_code_type_name(component_description)?;
                let domain_code = Self::domain_code_type_name(domain_description)?;

                gen.push_line(&format!(
                    "ZksyncError::{domain}({domain}::{component}(_)) => {{ Kind::{domain}({domain_code}::{component}) }},"
                ));
            }
        }

        for _ in 0..2 {
            gen.indent_less();
            gen.push_line("}");
        }

        gen.push_line(
            r#"
    pub fn get_code(&self) -> u32 {
        match self {"#,
        );

        gen.indent_more_by(2);
        for domain_description in domains() {
            for component_description in domain_description.components.values() {
                let domain = Self::domain_type_name(domain_description)?;
                let component = Self::component_type_name(component_description)?;
                let component_code = Self::component_code_type_name(component_description)?;
                gen.push_line(&format!("ZksyncError::{domain}({domain}::{component}(error)) => {{ Into::<{component_code}>::into(error) as u32 }},"));
            }
        }
        for _ in 0..3 {
            gen.indent_less();
            gen.push_line("}");
        }

        gen.push_line(
            r#"
impl std::fmt::Display for ZksyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self))
    }
}
impl IUnifiedError<ZksyncError> for ZksyncError {}
impl std::error::Error for ZksyncError {}
"#,
        );

        for domain_description in domains() {
            gen.push_block(&Self::define_domain_enum(domain_description)?);
        }

        for domain_description in domains() {
            for component_description in domain_description.components.values() {
                let domain = Self::domain_type_name(domain_description)?;
                let component = Self::component_type_name(component_description)?;
                gen.push_line(&format!(
                    r#"
impl ICustomError<ZksyncError, ZksyncError> for {component} {{
    fn to_unified(&self) -> ZksyncError {{
        ZksyncError::{domain}({domain}::{component}(self.clone()))
    }}
}}
"#
                ));
            }
        }

        Ok(File {
            relative_path: PathBuf::from("src/error/domains.rs"),
            content: gen.get_buffer(),
        })
    }
    fn define_domain_enum(
        domain_description: &DomainDescription,
    ) -> Result<String, GenerationError> {
        let mut gen = PrettyPrinter::default();
        let domain = Self::domain_type_name(domain_description)?;
        let domain_code = Self::domain_code_type_name(domain_description)?;

        gen.push_line(&format!(
            r#"
#[repr(u32)]
#[derive(AsRefStr, Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[strum_discriminants(name({domain_code}))]
#[strum_discriminants(derive(serde::Serialize, serde::Deserialize, FromRepr))]
#[strum_discriminants(vis(pub))]
pub enum {domain} {{"#
        ));

        gen.indent_more();

        for component in domain_description.components.values() {
            let component_name = Self::component_type_name(component)?;
            gen.push_line(&format!("{component_name}({component_name}),"));
        }
        gen.indent_less();
        gen.push_line("}");

        gen.push_line(&format!(
            r#"
impl {domain} {{
    pub fn get_name(&self) -> &str {{
        self.as_ref()
    }}
}}

"#
        ));

        Ok(gen.get_buffer())
    }
}
