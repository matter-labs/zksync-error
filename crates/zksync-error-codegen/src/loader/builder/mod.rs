pub mod context;
pub mod error;

use std::collections::BTreeMap;
use std::rc::Rc;

use context::ComponentTranslationContext;
use context::DomainTranslationContext;
use context::ErrorTranslationContext;
use context::ModelTranslationContext;
use context::TypeTranslationContext;
use error::MissingComponent;
use error::ModelBuildingError;
use error::TakeFromError;
use maplit::btreemap;
use zksync_error_model::validator::validate;

use crate::description::Collection;
use crate::loader::load;

use zksync_error_model::inner::ComponentDescription;
use zksync_error_model::inner::ComponentMetadata;
use zksync_error_model::inner::DomainDescription;
use zksync_error_model::inner::DomainMetadata;
use zksync_error_model::inner::ErrorDescription;
use zksync_error_model::inner::ErrorDocumentation;
use zksync_error_model::inner::ErrorName;
use zksync_error_model::inner::FieldDescription;
use zksync_error_model::inner::FullyQualifiedTargetLanguageType;
use zksync_error_model::inner::LikelyCause;
use zksync_error_model::inner::Model;
use zksync_error_model::inner::TargetLanguageType;
use zksync_error_model::inner::TypeDescription;
use zksync_error_model::inner::TypeMetadata;
use zksync_error_model::inner::VersionedOwner;
use zksync_error_model::merger::Merge as _;

use super::error::FileFormatError;
use super::error::LoadError;
use super::link::Link;

fn translate_type_bindings(
    value: &crate::description::ErrorNameMapping,
    error_name: &ErrorName,
) -> Result<BTreeMap<zksync_error_model::inner::LanguageName, TargetLanguageType>, ModelBuildingError>
{
    let mut result: BTreeMap<_, TargetLanguageType> = Default::default();
    let rust_name = match &value.rust {
        Some(crate::description::ErrorType { name }) => name,
        None => error_name,
    }
    .to_string();
    let typescript_name = match &value.typescript {
        Some(crate::description::ErrorType { name }) => name,
        None => error_name,
    }
    .to_string();

    result.insert("rust".into(), TargetLanguageType { name: rust_name });
    result.insert(
        "typescript".into(),
        TargetLanguageType {
            name: typescript_name,
        },
    );
    Ok(result)
}

fn translate_type_mappings(
    value: &crate::description::TypeMappings,
) -> Result<
    BTreeMap<zksync_error_model::inner::LanguageName, FullyQualifiedTargetLanguageType>,
    ModelBuildingError,
> {
    let mut result: BTreeMap<_, FullyQualifiedTargetLanguageType> = Default::default();
    if let Some(crate::description::FullyQualifiedType { name, path }) = &value.rust {
        result.insert(
            "rust".into(),
            FullyQualifiedTargetLanguageType {
                name: name.clone(),
                path: path.clone(),
            },
        );
    }
    Ok(result)
}

fn translate_type(
    value: &crate::description::Type,
    _ctx: &TypeTranslationContext,
) -> Result<TypeDescription, ModelBuildingError> {
    let crate::description::Type {
        name,
        description,
        bindings: codegen,
    } = value;
    Ok(TypeDescription {
        name: name.clone(),
        meta: TypeMetadata {
            description: description.clone(),
        },
        bindings: translate_type_mappings(codegen)?,
    })
}

fn translate_model(
    model: &crate::description::Root,
    ctx: ModelTranslationContext,
) -> Result<Model, ModelBuildingError> {
    let mut result = Model::default();
    let crate::description::Root { types, domains } = model;
    for t in types {
        let ctx = TypeTranslationContext {
            type_name: &t.name,
            parent: &ctx,
        };
        result
            .types
            .insert(t.name.clone(), translate_type(t, &ctx)?);
    }

    for domain in domains {
        let ctx = DomainTranslationContext { parent: &ctx };
        let transformed_domain: DomainDescription = translate_domain(domain, &ctx)?;
        result
            .domains
            .insert(transformed_domain.meta.name.clone(), transformed_domain);
    }

    Ok(result)
}

fn translate_field(
    value: &crate::description::Field,
) -> Result<FieldDescription, ModelBuildingError> {
    let crate::description::Field { name, r#type } = value;
    Ok(FieldDescription {
        name: name.clone(),
        r#type: r#type.clone(),
    })
}

fn translate_versioned_owner(
    owner: &Option<crate::description::VersionedOwner>,
) -> Result<Option<VersionedOwner>, ModelBuildingError> {
    Ok(owner.clone().map(
        |crate::description::VersionedOwner { name, version }| VersionedOwner { name, version },
    ))
}

fn structurize_likely_cause(cause: &str) -> crate::description::StructuredLikelyCause {
    crate::description::StructuredLikelyCause {
        cause: cause.to_owned(),
        fixes: vec![],
        report: "".into(),
        owner: None,
        references: vec![],
    }
}

fn translate_likely_cause(
    lc: &crate::description::LikelyCause,
) -> Result<LikelyCause, ModelBuildingError> {
    let crate::description::StructuredLikelyCause {
        cause,
        fixes,
        report,
        owner,
        references,
    } = match lc {
        crate::description::LikelyCause::Simple(str) => structurize_likely_cause(str),
        crate::description::LikelyCause::Structured(structured_likely_cause) => {
            structured_likely_cause.clone()
        }
    };
    Ok(LikelyCause {
        cause,
        fixes,
        report,
        owner: translate_versioned_owner(&owner)?,
        references,
    })
}

fn translate_error_documentation(
    doc: &crate::description::ErrorDocumentation,
) -> Result<ErrorDocumentation, ModelBuildingError> {
    let &crate::description::ErrorDocumentation {
        description,
        summary: short_description,
        likely_causes,
    } = &doc;

    let likely_causes: Vec<_> = likely_causes
        .iter()
        .flat_map(translate_likely_cause)
        .collect();

    Ok(ErrorDocumentation {
        description: description.clone(),
        summary: short_description.clone(),
        likely_causes,
    })
}

fn translate_error(
    error: &crate::description::Error,
    ctx: &ErrorTranslationContext,
) -> Result<ErrorDescription, ModelBuildingError> {
    let crate::description::Error {
        name,
        code,
        message,
        bindings,
        fields,
        doc,
    } = error;
    let transformed_fields: Result<_, _> = fields.iter().map(translate_field).collect();
    let transformed_bindings = translate_type_bindings(bindings, &error.name)?;

    let documentation = if let Some(doc) = doc {
        Some(translate_error_documentation(doc)?)
    } else {
        None
    };
    Ok(ErrorDescription {
        name: name.clone(),
        code: *code,
        message: message.clone(),
        fields: transformed_fields?,
        documentation,
        bindings: transformed_bindings,
        domain: ctx.parent.domain.clone(),
        component: ctx.component.clone(),
    })
}

enum FetchComponentResult {
    Errors(Vec<ErrorDescription>),
    Component(ComponentDescription),
}

fn fetch_named_component<'a>(
    link: &str,
    present_component_metadata: &Rc<ComponentMetadata>,
    ctx: &'a ComponentTranslationContext<'a>,
) -> Result<FetchComponentResult, TakeFromError> {
    let error_base = load(&Link::parse(link)?)?;
    match error_base {
        Collection::Root(_) | Collection::Domain(_) | Collection::Component(_) => {
            let component = error_base
                .get_component(&ctx.get_domain(), &present_component_metadata.name)
                .ok_or(MissingComponent {
                    domain_name: ctx.get_domain(),
                    component_name: present_component_metadata.name.to_owned(),
                })?;
            Ok(FetchComponentResult::Component(translate_component(
                component, ctx,
            )?))
        }
        Collection::Errors(errors) => Ok(FetchComponentResult::Errors(translate_errors(
            &errors,
            ctx,
            present_component_metadata,
        )?)),
    }
}

fn translate_errors<'a>(
    errors: &Vec<crate::description::Error>,
    ctx: &'a ComponentTranslationContext<'a>,
    component_meta: &Rc<ComponentMetadata>,
) -> Result<Vec<ErrorDescription>, ModelBuildingError> {
    let mut transformed_errors = Vec::default();

    for error in errors {
        let ctx = ErrorTranslationContext {
            parent: ctx,
            component: component_meta.clone(),
        };
        transformed_errors.push(translate_error(error, &ctx)?);
    }
    Ok(transformed_errors)
}
fn translate_component<'a>(
    component: &crate::description::Component,
    ctx: &'a ComponentTranslationContext<'a>,
) -> Result<ComponentDescription, ModelBuildingError> {
    let crate::description::Component {
        component_name,
        component_code,
        identifier_encoding,
        description,
        takeFrom,
        errors,
        bindings,
    } = component;

    let component_meta: Rc<ComponentMetadata> = Rc::new(ComponentMetadata {
        name: component_name.clone(),
        code: *component_code,
        bindings: maplit::btreemap! {
            "rust".into() => bindings.rust.clone().unwrap_or(component_name.clone()),
            "typescript".into() => bindings.typescript.clone().unwrap_or(component_name.clone()),
        },
        identifier: identifier_encoding.clone().unwrap_or_default(),
        description: description.clone().unwrap_or_default(),
        domain: ctx.domain.clone(),
    });

    let transformed_errors = translate_errors(errors, ctx, &component_meta)?;
    let mut result = ComponentDescription {
        meta: component_meta.clone(),
        errors: transformed_errors,
    };
    for take_from_address in takeFrom {
        match fetch_named_component(take_from_address, &component_meta, ctx)
            .map_err(|e| e.from_address(take_from_address))?
        {
            FetchComponentResult::Errors(vec) => result.errors.extend(vec),
            FetchComponentResult::Component(component_description) => {
                result
                    .merge(&component_description)
                    .map_err(|e| TakeFromError::MergeError(e).from_address(take_from_address))?;
            }
        };
    }

    Ok(result)
}

fn translate_domain<'a>(
    value: &crate::description::Domain,
    ctx: &'a DomainTranslationContext<'a>,
) -> Result<DomainDescription, ModelBuildingError> {
    let crate::description::Domain {
        domain_name,
        domain_code,
        identifier_encoding,
        description,
        components,
        bindings,
    } = value;
    let mut new_components: BTreeMap<_, _> = BTreeMap::default();
    let metadata = Rc::new(DomainMetadata {
        name: domain_name.clone(),
        code: *domain_code,
        identifier: identifier_encoding.clone().unwrap_or_default(),
        description: description.clone().unwrap_or_default(),
        bindings: btreemap! {
            "rust".into() => bindings.rust.clone().unwrap_or(domain_name.clone()),
            "typescript".into() => bindings.typescript.clone().unwrap_or(domain_name.clone()),
        },
    });
    for component in components {
        let ctx = ComponentTranslationContext {
            domain: metadata.clone(),
            parent: ctx,
        };

        let translated_component = translate_component(component, &ctx)?;
        new_components.insert(translated_component.meta.name.clone(), translated_component);
    }
    Ok(DomainDescription {
        meta: metadata,
        components: new_components,
    })
}

fn load_root_model(root_link: &Link) -> Result<Model, LoadError> {
    let source = root_link.clone();
    match load(root_link)? {
        Collection::Domain(_) => Err(LoadError::FileFormatError(
            FileFormatError::ExpectedFullGotDomain { origin: source },
        )),
        Collection::Component(_) => Err(LoadError::FileFormatError(
            FileFormatError::ExpectedFullGotComponent { origin: source },
        )),
        Collection::Errors(_) => Err(LoadError::FileFormatError(
            FileFormatError::ExpectedFullGotComponent { origin: source },
        )),
        Collection::Root(root) => Ok(translate_model(
            &root,
            ModelTranslationContext { origin: source },
        )?),
    }
}

fn add_default_error(model: &mut Model) {
    for domain in model.domains.values_mut() {
        for component in domain.components.values_mut() {
            if !component.errors.iter().any(|e| e.code == 0) {
                component.errors.push(ErrorDescription {
                    domain: domain.meta.clone(),
                    component: component.meta.clone(),
                    name: "GenericError".into(),
                    code: 0,
                    message: "Generic error: {message}".into(),
                    fields: vec![FieldDescription {
                        name: "message".into(),
                        r#type: "string".into(),
                    }],
                    documentation: None,
                    bindings: btreemap! {
                        "rust".into() => TargetLanguageType { name: "GenericError".into()} ,
                        "typescript".into() => TargetLanguageType { name: "GenericError".into()} ,
                    },
                });
            }
        }
    }
}

fn bind_error_types(model: &mut Model) {
    fn error_name(component_name: &str) -> String {
        format!("Box<{component_name}>")
    }
    for domain in model.domains.values() {
        for component in domain.components.values() {
            let bindings: BTreeMap<_, zksync_error_model::inner::FullyQualifiedTargetLanguageType> =
                component
                    .meta
                    .bindings
                    .iter()
                    .map(|(k, v)| (k.to_owned(), error_name(v).as_str().into()))
                    .collect();
            let value = TypeDescription {
                name: component.meta.name.clone(),
                meta: TypeMetadata {
                    description: component.meta.description.clone(),
                },
                bindings,
            };
            model.types.insert(component.meta.name.clone(), value);
        }
    }
}

pub fn build_model(
    root_link: &Link,
    additions: &Vec<Link>,
    diagnostic: bool,
) -> Result<Model, ModelBuildingError> {
    let mut root_model = load_root_model(root_link)?;

    for input_link in additions {
        let part = load_root_model(input_link)?;
        root_model
            .merge(&part)
            .map_err(|error| ModelBuildingError::MergeError {
                merge_error: error,
                main_model_origin: root_link.clone(),
                additional_model_origin: input_link.clone(),
            })?
    }

    add_default_error(&mut root_model);
    bind_error_types(&mut root_model);
    if diagnostic {
        eprintln!("Model: {root_model:#?}");
        eprintln!("Model validation...");
    }

    validate(&root_model)?;
    Ok(root_model)
}
