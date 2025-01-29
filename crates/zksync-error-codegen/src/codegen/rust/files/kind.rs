use std::path::PathBuf;

use crate::codegen::printer::PrettyPrinter;
use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_kind(&mut self) -> Result<File, GenerationError> {
        let domains = || self.model.domains.values();

        let mut gen = PrettyPrinter::default();
        Self::preamble(&mut gen);
        gen.push_line(
            r#"
use strum_macros::{EnumDiscriminants, FromRepr};
"#,
        );

        for domain_description in domains() {
            let domain_code_type = Self::domain_code_type_name(domain_description)?;
            gen.push_line(&format!("use crate::error::domains::{domain_code_type};"));
        }

        gen.push_line(
            r#"
#[derive(Clone, Debug, EnumDiscriminants, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[strum_discriminants(name(DomainCode))]
#[strum_discriminants(derive(FromRepr))]
#[strum_discriminants(vis(pub))]
#[repr(u32)]
pub enum Kind {"#,
        );
        gen.indent_more();

        for domain_description in domains() {
            let domain = Self::domain_type_name(domain_description)?;
            let domain_code_type = Self::domain_code_type_name(domain_description)?;
            let domain_code_value = domain_description.meta.code;
            gen.push_line(&format!(
                "{domain}({domain_code_type}) = {domain_code_value},"
            ));
        }

        // Compiler(CompilerCode) = 2,
        // Tooling(ToolingCode) = 3,

        gen.indent_less();
        gen.push_line(
            r#"}

impl Kind {
    pub fn domain_code(&self) -> u32 {
        let domain: DomainCode = self.clone().into();
        domain as u32
    }
    pub fn component_code(&self) -> u32 {
        match self {"#,
        );

        gen.indent_more_by(3);

        for domain_description in domains() {
            let domain = Self::domain_type_name(domain_description)?;
            gen.push_line(&format!(
                "Kind::{domain}(component) => component.clone() as u32,"
            ));
        }
        gen.indent_less_by(3);
        gen.push_line(
            r#"
        }
    }
}"#,
        );

        Ok(File {
            relative_path: PathBuf::from("src/kind.rs"),
            content: gen.get_buffer(),
        })
    }
}
