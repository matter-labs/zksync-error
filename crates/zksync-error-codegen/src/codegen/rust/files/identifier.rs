use std::path::PathBuf;

use crate::codegen::printer::PrettyPrinter;
use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_identifier(&mut self) -> Result<File, GenerationError> {
        let domains = || self.model.domains.values();
        let mut gen = PrettyPrinter::default();
        Self::preamble(&mut gen);

        for domain_description in domains() {
            let domain_code = Self::domain_code_type_name(domain_description)?;
            gen.push_line(&format!("use crate::error::domains::{domain_code};"));
        }

        gen.push_line(
            r#"
use crate::error::NamedError;
use crate::kind::DomainCode;
use crate::kind::Kind;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StructuredErrorCode {
    pub domain_code: u32,
    pub component_code: u32,
    pub error_code: u32,
}

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


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Identifier {
    pub kind: Kind,
    pub code: u32,
}

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
"#,
        );

        gen.indent_more_by(3);

        let domains = || self.model.domains.values();

        for domain_description in domains() {
            let domain = Self::domain_type_name(domain_description)?;
            let domain_code = Self::domain_code_type_name(domain_description)?;

            gen.push_line(&format!("DomainCode::{domain} => {{"));
            gen.indent_more();

            gen.push_line(&format!(
                "Kind::{domain}({domain_code}::from_repr(component_code)?)"
            ));

            gen.indent_less();
            gen.push_line("},");
        }
        gen.indent_less_by(3);
        gen.push_line(
            r#"
       };
        Some(Identifier { kind, code: error_code })
    }
}
"#,
        );

        gen.push_str(
            r#"
trait Identifying {
   fn get_identifier_repr(&self)-> String;
}

"#,
        );

        gen.push_str(
            r#"
impl Identifying for Kind {
  fn get_identifier_repr(&self) -> String {
    match self {
"#,
        );

        gen.indent_more_by(3);

        for domain_description in domains() {
            let domain_code = Self::domain_code_type_name(domain_description)?;
            let domain_name = Self::domain_type_name(domain_description)?;
            for component_description in domain_description.components.values() {
                let component_name = Self::component_type_name(component_description)?;
                let domain_contribution = &domain_description.meta.identifier;
                let component_contribution = &component_description.meta.identifier;
                gen.push_line(&format!(
                    r#"Kind::{domain_name}({domain_code}::{component_name}) => "{domain_contribution}-{component_contribution}","#
                ));
            }
        }

        gen.indent_less_by(3);
        gen.push_str(
            r#"
    }.into()
  }
}
"#,
        );
        gen.push_str(
            r#"
impl Identifying for Identifier {
    fn get_identifier_repr(&self) -> String {
       format!("[{}-{}]", self.kind.get_identifier_repr(), self.code)
  }
}
"#,
        );

        gen.push_str(
            r#"
impl NamedError for Identifier {
    fn get_error_name(&self) -> String {
        match self.kind {
"#,
        );
        gen.indent_more_by(3);

        for domain_description in domains() {
            let domain_code = Self::domain_code_type_name(domain_description)?;
            let domain_name = Self::domain_type_name(domain_description)?;
            for component_description in domain_description.components.values() {
                let component_name = Self::component_type_name(component_description)?;
                let component_code = Self::component_code_type_name(component_description)?;
                gen.push_line(&format!(
                    r#"Kind::{domain_name}({domain_code}::{component_name}) => crate::error::definitions::{component_code}::from_repr(self.code).expect("Internal error").get_error_name(),"#));
            }
        }

        gen.indent_less_by(3);
        gen.push_line(
            r#"
        }
    }
}
"#,
        );
        gen.push_str(r#"
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
"#,
        );

        Ok(File {
            content: gen.get_buffer(),
            relative_path: PathBuf::from("src/identifier.rs"),
        })
    }
}
