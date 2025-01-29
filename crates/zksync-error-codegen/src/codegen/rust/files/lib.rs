use std::path::PathBuf;

use crate::codegen::printer::PrettyPrinter;
use crate::codegen::rust::error::GenerationError;
use crate::codegen::rust::RustBackend;
use crate::codegen::File;

fn replace_non_alphanumeric(input: &str, replacement: char) -> String {
    input
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { replacement })
        .collect()
}

impl RustBackend {
    pub fn generate_file_lib(&mut self) -> Result<File, GenerationError> {
        let mut gen = PrettyPrinter::default();
        Self::preamble(&mut gen);
        gen.push_line(
            r#"
#![allow(unused)]

pub mod error;
pub mod identifier;
pub mod kind;
pub mod packed;
pub mod serialized;
pub mod untyped;
pub mod documentation;


pub use crate::error::domains::ZksyncError;
"#,
        );

        fn sanitize(s: &str) -> String {
            replace_non_alphanumeric(s, '_')
        }

        for domain in self.model.domains.values() {
            let outer_module = sanitize(&domain.meta.identifier);
            gen.push_line(&format!("pub mod {outer_module} {{"));
            gen.indent_more();
            for component in domain.components.values() {
                let inner_module = sanitize(&component.meta.identifier);
                gen.push_line(&format!("pub mod {inner_module} {{",));
                gen.indent_more();

                let enum_name = component
                    .meta
                    .bindings
                    .get("rust")
                    .expect("Internal model error");

                gen.push_line(&format!(
                    "pub use crate::error::definitions::{enum_name} as {enum_name}Error;"
                ));

                for error in &component.errors {
                    let enum_variant = sanitize(
                        &error
                            .bindings
                            .get("rust")
                            .expect("Internal model error")
                            .name,
                    );
                    gen.push_line(&format!(
                        "pub use crate::error::definitions::{enum_name}::{enum_variant};"
                    ));
                }

                gen.push_block(&format!(
                    r#"
#[macro_export]
macro_rules! {outer_module}_{inner_module}_generic_error {{
    ($($arg:tt)*) => {{
        zksync_error::error::definitions::{enum_name}::GenericError {{ message: format!($($arg)*) }}
    }};
}}
pub use crate::{outer_module}_{inner_module}_generic_error as generic_error;
"#
                ));

                gen.push_line("}");
                gen.indent_less();
            }
            gen.push_line("}");
            gen.indent_less();
        }
        Ok(File {
            content: gen.get_buffer(),
            relative_path: PathBuf::from("src/lib.rs"),
        })
    }
}
