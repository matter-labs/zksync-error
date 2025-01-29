use std::path::PathBuf;

use crate::codegen::printer::PrettyPrinter;
use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

impl RustBackend {
    pub fn generate_file_untyped(&mut self) -> Result<File, GenerationError> {
        let mut gen = PrettyPrinter::default();
        Self::preamble(&mut gen);
        gen.push_line(
            r#"
use crate::identifier::Identifier;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UntypedErrorObject {
    pub identifier: Identifier,
    pub name: String,
    pub fields: serde_json::Map<String, serde_json::Value>, // Specific value introduced by user; unpacked from the Domain/subdomain and error name
    pub raw: serde_json::Value, // Specific value introduced by user; unpacked from the Domain/subdomain.
}
"#,
        );

        Ok(File {
            content: gen.get_buffer(),
            relative_path: PathBuf::from("src/untyped.rs"),
        })
    }
}
