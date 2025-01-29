pub mod config;
pub mod error;

use std::path::PathBuf;

use config::MDBookBackendConfig;
use error::GenerationError;
use include_dir::Dir;
use tera::Tera;

use super::Backend;
use super::File;

use include_dir::include_dir;
use zksync_error_model::inner::Model;
use zksync_error_model::unpacked::flatten;
use zksync_error_model::unpacked::UnpackedModel;

pub struct MDBookBackend {
    model: Model,
}

impl MDBookBackend {
    pub fn new(model: &Model) -> Self {
        Self {
            model: model.clone(),
        }
    }
}

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/doc_templates/mdbook");

fn initialize_tera() -> Result<Tera, <MDBookBackend as Backend<MDBookBackendConfig>>::Error> {
    let mut tera = Tera::default();
    for file in TEMPLATES_DIR.files() {
        if let Some(path) = file.path().to_str() {
            if let Ok(contents) = std::str::from_utf8(file.contents()) {
                tera.add_raw_template(path, contents)?;
            }
        }
    }
    Ok(tera)
}

impl MDBookBackend {
    fn copy_css(&mut self, _config: &MDBookBackendConfig) -> Result<File, GenerationError> {
        let filename = "css/version-box.css";
        let content = TEMPLATES_DIR
            .get_file(filename)
            .unwrap_or_else(|| panic!("Missing file `{filename}`"))
            .contents_utf8()
            .unwrap_or_else(|| {
                panic!("Internal error: decoding utf-8 string from file {filename}.")
            });

        Ok(File {
            relative_path: PathBuf::from(filename),
            content: content.into(),
        })
    }
    fn copy_js(&mut self, _config: &MDBookBackendConfig) -> Result<File, GenerationError> {
        let filename = "js/version-box.js";
        let content = TEMPLATES_DIR
            .get_file(filename)
            .unwrap_or_else(|| panic!("Missing file `{filename}`"))
            .contents_utf8()
            .unwrap_or_else(|| {
                panic!("Internal error: decoding utf-8 string from file {filename}.")
            });

        Ok(File {
            relative_path: PathBuf::from(filename),
            content: content.into(),
        })
    }

    fn copy_as_is(
        &mut self,
        filename: &str,
        _config: &MDBookBackendConfig,
    ) -> Result<File, GenerationError> {
        let content = TEMPLATES_DIR
            .get_file(filename)
            .unwrap_or_else(|| panic!("Missing file `{filename}`"))
            .contents_utf8()
            .unwrap_or_else(|| {
                panic!("Internal error: decoding utf-8 string from file {filename}.")
            });

        eprintln!("Copy as is: {filename}: \n{content}");
        Ok(File {
            relative_path: PathBuf::from(filename),
            content: content.into(),
        })
    }

    fn generate_summary(
        &mut self,
        tera: &Tera,
        model: &UnpackedModel,
        _config: &MDBookBackendConfig,
    ) -> Result<File, GenerationError> {
        let mut context = tera::Context::new();
        context.insert("domains", &model.domains.values().collect::<Vec<_>>());
        context.insert("components", &model.components.values().collect::<Vec<_>>());
        context.insert("errors", &model.errors.values().collect::<Vec<_>>());
        let content = tera.render("SUMMARY.md", &context)?;

        Ok(File {
            relative_path: PathBuf::from("src/SUMMARY.md"),
            content,
        })
    }

    fn generate_component(
        &mut self,
        tera: &Tera,
        component: &zksync_error_model::unpacked::ComponentMetadata,
        model: &UnpackedModel,
        _config: &MDBookBackendConfig,
    ) -> Result<File, GenerationError> {
        let mut context = tera::Context::new();
        context.insert("component", component);
        context.insert("components", &model.components.values().collect::<Vec<_>>());
        context.insert("errors", &model.errors.values().collect::<Vec<_>>());
        let content = tera.render("component.md", &context)?;
        let domain_name = &component.domain_name;
        let component_name = &component.name;

        Ok(File {
            relative_path: PathBuf::from(format!(
                "src/domains/{domain_name}/{component_name}/README.md"
            )),
            content,
        })
    }
    fn generate_domain(
        &mut self,
        tera: &Tera,
        domain: &zksync_error_model::unpacked::DomainMetadata,
        model: &UnpackedModel,
        _config: &MDBookBackendConfig,
    ) -> Result<File, GenerationError> {
        let mut context = tera::Context::new();
        context.insert("domain", domain);
        context.insert("components", &model.components.values().collect::<Vec<_>>());
        context.insert("errors", &model.errors.values().collect::<Vec<_>>());
        let content = tera.render("domain.md", &context)?;
        let domain_name = &domain.name;

        Ok(File {
            relative_path: PathBuf::from(format!("src/domains/{domain_name}/README.md")),
            content,
        })
    }

    fn generate_error(
        &mut self,
        tera: &Tera,
        domain: &zksync_error_model::unpacked::DomainMetadata,
        component: &zksync_error_model::unpacked::ComponentMetadata,
        error: &zksync_error_model::unpacked::ErrorDescription,
        model: &UnpackedModel,
        _config: &MDBookBackendConfig,
    ) -> Result<File, GenerationError> {
        let mut context = tera::Context::new();
        context.insert("domain", domain);
        context.insert("components", &model.components.values().collect::<Vec<_>>());
        context.insert("errors", &model.errors.values().collect::<Vec<_>>());
        context.insert("error", error);
        let content = tera.render("error.md", &context)?;
        let domain_name = &domain.name;
        let component_name = &component.name;
        let error_name = &error.name;

        Ok(File {
            relative_path: PathBuf::from(format!(
                "src/domains/{domain_name}/{component_name}/{error_name}.md"
            )),
            content,
        })
    }
}
impl Backend<MDBookBackendConfig> for MDBookBackend {
    type Error = GenerationError;

    fn get_name() -> &'static str {
        "markdown-mdbook"
    }

    fn generate(&mut self, _config: &MDBookBackendConfig) -> Result<Vec<File>, Self::Error> {
        let tera = initialize_tera()?;

        let model = flatten(&self.model);
        let mut results = vec![
            self.generate_summary(&tera, &model, _config)?,
            self.copy_as_is("book.toml", _config)?,
            self.copy_as_is("css/version-box.css", _config)?,
            self.copy_as_is("js/version-box.js", _config)?,
        ];

        for domain in model.domains.values() {
            results.push(self.generate_domain(&tera, domain, &model, _config)?);
            for component in model.components.values() {
                if component.domain_name == domain.name {
                    results.push(self.generate_component(&tera, component, &model, _config)?);
                    for error in model.errors.values() {
                        if error.component == component.name {
                            results.push(self.generate_error(
                                &tera, domain, component, error, &model, _config,
                            )?);
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    fn get_language_name() -> &'static str {
        "markdown"
    }
}
