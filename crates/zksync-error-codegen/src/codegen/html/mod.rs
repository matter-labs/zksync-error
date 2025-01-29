pub mod config;
pub mod error;

use std::path::PathBuf;

use config::HtmlBackendConfig;
use error::GenerationError;
use include_dir::Dir;
use tera::Tera;

use super::Backend;
use super::File;

use include_dir::include_dir;
use zksync_error_model::inner::Model;
use zksync_error_model::unpacked::flatten;
use zksync_error_model::unpacked::UnpackedModel;

pub struct HtmlBackend {
    model: UnpackedModel,
}

impl HtmlBackend {
    pub fn new(model: &Model) -> Self {
        Self {
            model: flatten(model),
        }
    }
}

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/doc_templates/html");

impl Backend<HtmlBackendConfig> for HtmlBackend {
    type Error = GenerationError;

    fn get_name() -> &'static str {
        "html-doc"
    }

    fn generate(&mut self, _config: &HtmlBackendConfig) -> Result<Vec<File>, Self::Error> {
        let mut tera = Tera::default();
        for file in TEMPLATES_DIR.files() {
            if let Some(path) = file.path().to_str() {
                if let Ok(contents) = std::str::from_utf8(file.contents()) {
                    tera.add_raw_template(path, contents)?;
                }
            }
        }

        let mut results = vec![];
        for error in self.model.errors.values() {
            // Create context for Tera
            let mut context = tera::Context::new();
            context.insert("error", &error);

            let content = tera.render("error.html", &context)?;
            let domain_name = &error.domain;
            let component_name = &error.component;
            let error_name = &error.name;
            results.push(File {
                relative_path: PathBuf::from(format!(
                    "{domain_name}/{component_name}/{error_name}.html"
                )),
                content,
            });
        }

        results.push({
            let mut context = tera::Context::new();

            context.insert("errors", &self.model.errors.values().collect::<Vec<_>>());
            context.insert(
                "components",
                &self.model.components.values().collect::<Vec<_>>(),
            );
            context.insert("domains", &self.model.domains.values().collect::<Vec<_>>());

            let content = tera.render("index.html", &context)?;
            File {
                relative_path: PathBuf::from("index.html"),
                content,
            }
        });

        Ok(results)
    }

    fn get_language_name() -> &'static str {
        "html"
    }
}
