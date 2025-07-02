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
use zksync_error_model::unpacked::UnpackedModel;
use zksync_error_model::unpacked::flatten;

pub struct MDBookBackend {
    _config: MDBookBackendConfig,
    model: Model,
}

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/doc_templates/mdbook");

fn initialize_tera() -> Result<Tera, <MDBookBackend as Backend>::GenerationError> {
    let mut tera = Tera::default();
    for file in TEMPLATES_DIR.files() {
        if let Some(path) = file.path().to_str()
            && let Ok(contents) = std::str::from_utf8(file.contents())
        {
            tera.add_raw_template(path, contents)?;
        }
    }
    Ok(tera)
}

impl MDBookBackend {
    fn copy_as_is(&mut self, filename: &str) -> Result<File, GenerationError> {
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
    ) -> Result<File, GenerationError> {
        let mut context = tera::Context::new();
        context.insert("component", component);
        context.insert("components", &model.components.values().collect::<Vec<_>>());
        context.insert("errors", &model.errors.values().collect::<Vec<_>>());
        let content = tera.render("component.md", &context)?;
        let domain_name = &component.domain_name;
        let component_name = &component.identifier.name;

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
    ) -> Result<File, GenerationError> {
        let mut context = tera::Context::new();
        context.insert("domain", domain);
        context.insert("components", &model.components.values().collect::<Vec<_>>());
        context.insert("errors", &model.errors.values().collect::<Vec<_>>());
        let content = tera.render("domain.md", &context)?;
        let domain_name = &domain.identifier.name;

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
    ) -> Result<File, GenerationError> {
        let mut context = tera::Context::new();
        context.insert("domain", domain);
        context.insert("components", &model.components.values().collect::<Vec<_>>());
        context.insert("errors", &model.errors.values().collect::<Vec<_>>());
        context.insert("error", error);
        let content = tera.render("error.md", &context)?;
        let domain_name = &domain.identifier.name;
        let component_name = &component.identifier.name;
        let error_name = &error.name;

        Ok(File {
            relative_path: PathBuf::from(format!(
                "src/domains/{domain_name}/{component_name}/{error_name}.md"
            )),
            content,
        })
    }
}
impl Backend for MDBookBackend {
    type Config = MDBookBackendConfig;
    type GenerationError = GenerationError;

    fn new(config: Self::Config, model: &Model) -> Self {
        Self {
            _config: config,
            model: model.clone(),
        }
    }

    fn get_name() -> &'static str {
        "markdown-mdbook"
    }

    fn get_language_name() -> &'static str {
        "markdown"
    }

    fn generate(&mut self) -> Result<Vec<File>, Self::GenerationError> {
        let tera = initialize_tera()?;

        let model = flatten(&self.model);
        let mut results = vec![
            self.generate_summary(&tera, &model)?,
            self.copy_as_is("book.toml")?,
            self.copy_as_is("css/version-box.css")?,
            self.copy_as_is("js/version-box.js")?,
        ];

        for domain in model.domains.values() {
            results.push(self.generate_domain(&tera, domain, &model)?);
            for component in model.components.values() {
                if component.domain_name == domain.identifier.name {
                    results.push(self.generate_component(&tera, component, &model)?);
                    for error in model.errors.values() {
                        if error.component == component.identifier.name {
                            results.push(
                                self.generate_error(&tera, domain, component, error, &model)?,
                            );
                        }
                    }
                }
            }
        }

        Ok(results)
    }
}
