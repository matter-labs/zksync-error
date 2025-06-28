#![allow(unused)]

pub mod context;
pub mod error;

use std::collections::BTreeMap;
use std::iter;
use std::rc::Rc;

use context::ComponentTranslationContext;
use context::DomainTranslationContext;
use context::ErrorTranslationContext;
use context::ModelTranslationContext;
use context::TypeTranslationContext;
use error::ModelBuildingError;
use maplit::btreemap;
use zksync_error_model::inner::domain;
use zksync_error_model::link::Link;

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
use zksync_error_model::validator::validate;

use crate::description::Root;
use crate::description::merge::Mergeable as _;

use super::NormalizedDescriptionFragment;
use super::error::LoadError;
use super::get_resolution_context;
use super::load_fragments_multiple_sources;
use super::resolution::overrides::Remapping;

fn add_missing<U, S>(map: &mut BTreeMap<String, U>, default: U, keys: impl Iterator<Item = S>)
where
    U: Clone,
    S: Into<String>,
{
    for key in keys {
        let _ = map.entry(key.into()).or_insert(default.clone());
    }
}

fn ensure_existing<U, S>(
    map: BTreeMap<String, U>,
    default: U,
    keys: impl Iterator<Item = S>,
) -> BTreeMap<String, U>
where
    U: Clone,
    S: Into<String>,
{
    let mut result = map.clone();
    add_missing(&mut result, default, keys);
    result
}
fn translate_and_populate_bindings(
    bindings: &BTreeMap<String, String>,
    default: &str,
) -> BTreeMap<String, String> {
    ensure_existing(
        bindings.clone(),
        default.to_string(),
        ["rust", "typescript"].into_iter(),
    )
}

fn translate_type_bindings(
    value: &crate::description::ErrorNameMapping,
    error_name: &ErrorName,
) -> Result<BTreeMap<zksync_error_model::inner::LanguageName, TargetLanguageType>, ModelBuildingError>
{
    let result: BTreeMap<_, _> = value
        .iter()
        .map(|(language_name, mapping)| {
            (
                language_name.clone(),
                TargetLanguageType {
                    expression: mapping.name.clone(),
                },
            )
        })
        .collect();
    Ok(ensure_existing(
        result,
        TargetLanguageType {
            expression: error_name.clone(),
        },
        ["rust", "typescript"].into_iter(),
    ))
}

fn translate_type_mappings(
    value: &crate::description::TypeMappings,
) -> Result<
    BTreeMap<zksync_error_model::inner::LanguageName, FullyQualifiedTargetLanguageType>,
    ModelBuildingError,
> {
    Ok(value
        .iter()
        .map(|(language_name, mapping)| {
            (
                language_name.clone(),
                FullyQualifiedTargetLanguageType {
                    expression: mapping.expression.clone(),
                },
            )
        })
        .collect())
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
            description: description.clone().into(),
        },
        bindings: translate_type_mappings(codegen)?,
    })
}

fn translate_model(
    model: &crate::description::Root,
    ctx: ModelTranslationContext,
) -> Result<Model, ModelBuildingError> {
    let mut result = Model::default();
    let crate::description::Root {
        types,
        domains,
        take_from: _,
    } = model;
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
        result.domains.insert(
            transformed_domain.meta.identifier.name.clone(),
            transformed_domain,
        );
    }

    Ok(result)
}

fn translate_field(
    value: &crate::description::Field,
) -> Result<FieldDescription, ModelBuildingError> {
    let crate::description::Field {
        name,
        r#type,
        comment: _,
    } = value;
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
        description: description.clone().into(),
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
        origins,
        comment: _,
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
        origins: origins.clone(),
    })
}

fn translate_errors<'a>(
    errors: &Vec<crate::description::Error>,
    ctx: &'a ComponentTranslationContext<'a>,
    component_meta: &Rc<ComponentMetadata>,
) -> Result<Vec<ErrorDescription>, ModelBuildingError> {
    let ctx = ErrorTranslationContext {
        parent: ctx,
        component: component_meta.clone(),
    };

    let mut transformed_errors = Vec::default();
    for error in errors {
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
        take_from,
        errors,
        bindings,
        origins,
        comment: _,
    } = component;

    let new_bindings = translate_and_populate_bindings(bindings, component_name);
    let component_meta: Rc<ComponentMetadata> = Rc::new(ComponentMetadata {
        bindings: new_bindings,
        identifier: zksync_error_model::inner::component::Identifier {
            name: component_name.clone(),
            code: *component_code,
            encoding: identifier_encoding.clone().unwrap_or_default(),
        },
        description: description.clone().unwrap_or_default().into(),
        domain: ctx.domain.clone(),
        origins: origins.clone(),
    });

    let transformed_errors = translate_errors(errors, ctx, &component_meta)?;
    let mut result = ComponentDescription {
        meta: component_meta.clone(),
        errors: transformed_errors,
    };

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
        take_from,
        origins,
        comment: _,
    } = value;
    let mut new_components: BTreeMap<_, _> = BTreeMap::default();
    let metadata = Rc::new(DomainMetadata {
        identifier: zksync_error_model::inner::domain::Identifier {
            name: domain_name.clone(),
            code: *domain_code,
            encoding: identifier_encoding.clone().unwrap_or_default(),
        },
        description: description.clone().unwrap_or_default().into(),
        bindings: translate_and_populate_bindings(bindings, domain_name),
        origins: origins.clone(),
    });

    {
        let ctx = ComponentTranslationContext {
            domain: metadata.clone(),
            parent: ctx,
        };
        for component in components {
            let translated_component = translate_component(component, &ctx)?;
            new_components.insert(
                translated_component.meta.identifier.name.clone(),
                translated_component,
            );
        }
    }

    let mut result = DomainDescription {
        meta: metadata,
        components: new_components,
    };

    Ok(result)
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
                        "rust".into() => TargetLanguageType { expression: "GenericError".into()} ,
                        "typescript".into() => TargetLanguageType { expression: "GenericError".into()} ,
                    },
                    origins: vec![],
                });
            }
        }
    }
}
fn bind_error_types(model: &mut Model) {
    use zksync_error_model::inner::FullyQualifiedTargetLanguageType as TargetType;

    fn error_binding(language: &str, binding: &str) -> Option<(String, TargetType)> {
        match language {
            "rust" => Some((
                language.to_owned(),
                format!("Box<{binding}>").as_str().into(),
            )),
            _ => None,
        }
    }

    fn create_type_description(
        name: String,
        description: String,
        bindings: &BTreeMap<String, String>,
    ) -> TypeDescription {
        TypeDescription {
            name,
            meta: TypeMetadata { description },
            bindings: bindings
                .iter()
                .filter_map(|(lang, binding)| error_binding(lang, binding))
                .collect(),
        }
    }

    for domain in model.domains.values() {
        let domain_type = create_type_description(
            domain.meta.identifier.name.clone(),
            domain.meta.description.clone(),
            &domain.meta.bindings,
        );

        model.types.insert(domain_type.name.clone(), domain_type);

        for component in domain.components.values() {
            let component_type = create_type_description(
                component.meta.identifier.name.clone(),
                component.meta.description.clone(),
                &component.meta.bindings,
            );
            model
                .types
                .insert(component_type.name.clone(), component_type);
        }
    }
}

pub fn build_model(
    sources: Vec<Link>,
    overrides: Remapping,
    diagnostic: bool,
) -> Result<Model, ModelBuildingError> {
    let mut resolution_context = get_resolution_context(overrides);
    let collection = load_fragments_multiple_sources(sources.into_iter(), &mut resolution_context)?;

    let acc = {
        let mut acc = Root::default();

        for fragment in collection {
            acc = acc
                .merge(fragment.root)
                .map_err(|inner| ModelBuildingError::MergeError {
                    inner: Box::new(inner),
                    origin: fragment.origin,
                })?;
        }
        acc
    };

    if diagnostic {
        eprintln!("\n --- Combined description ---\n{acc}")
    }

    let mut root_model = translate_model(&acc, ModelTranslationContext)?;

    add_default_error(&mut root_model);
    bind_error_types(&mut root_model);
    validate(&root_model)?;

    if diagnostic {
        eprintln!("Model: {root_model:#?}");
    }

    Ok(root_model)
}
