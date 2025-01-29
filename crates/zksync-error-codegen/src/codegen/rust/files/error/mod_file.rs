use std::path::PathBuf;

use crate::codegen::printer::PrettyPrinter;
use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_error_mod(&mut self) -> Result<File, GenerationError> {
        let domains = || self.model.domains.values();

        let mut gen = PrettyPrinter::default();
        Self::preamble(&mut gen);
        gen.push_line(
            r#"
pub mod definitions;
pub mod domains;

use std::error::Error;
use crate::identifier::Identifier;
use crate::error::domains::ZksyncError;
"#,
        );

        for domain_description in domains() {
            let domain = Self::domain_type_name(domain_description)?;
            gen.push_line(&format!("use crate::error::domains::{domain};"));
        }

        gen.push_line(
            r#"
pub trait IError<ContainedType>: Error
where
    ContainedType: Clone,
{
    fn get_identifier(&self) -> Identifier;
    fn get_message(&self) -> String;
    fn get_data(&self) -> ContainedType;
}

pub trait IUnifiedError<ContainedType>:
    serde::Serialize + for<'de> serde::Deserialize<'de> + IError<ContainedType>
where
    ContainedType: Clone,
{
}

pub trait ICustomError<U, C>
where
    U: IUnifiedError<C>,
    C: Clone,
{
    fn to_unified(&self) -> U;
}

pub trait CustomErrorMessage {
    fn get_message(&self) -> String;
}

pub trait NamedError {
    fn get_error_name(&self) -> String;
}

impl IError<ZksyncError> for ZksyncError {
    fn get_identifier(&self) -> Identifier {
        Identifier {
            kind: self.get_kind(),
            code: self.get_code(),
        }
    }

    fn get_message(&self) -> String {
        match self {"#,
        );

        gen.indent_more_by(3);

        let domains = || self.model.domains.values();

        for domain_description in domains() {
            for component_description in domain_description.components.values() {
                let domain = Self::domain_type_name(domain_description)?;
                let component = Self::component_type_name(component_description)?;
                gen.push_line(&format!(
                    "ZksyncError::{domain}({domain}::{component}(error)) => error.get_message(),"
                ));
            }
        }
        gen.indent_less_by(3);
        gen.push_line(
            r#"
        }
    }

    fn get_data(&self) -> ZksyncError {
        self.clone()
    }
}"#,
        );
        Ok(File {
            content: gen.get_buffer(),
            relative_path: PathBuf::from("src/error/mod.rs"),
        })
    }
}
