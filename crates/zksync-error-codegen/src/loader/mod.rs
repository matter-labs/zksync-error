use std::collections::BTreeSet;

use error::LoadError;
use fetch::LoadResult;
use fetch::load_text;
use resolution::ResolutionContext;
use zksync_error_model::link::Link;

use crate::description::HierarchyFragment;
use crate::description::Root;
use crate::description::accessors::annotate_origins;
use crate::description::error::FileFormatError;
use crate::description::normalization::binding::BindingPoint;
use crate::description::normalization::produce_root;

pub mod builder;
pub mod cargo;
pub mod error;
pub mod fetch;
pub mod resolution;

///
/// A fragment of a model, loaded through a link.
/// - No default values are assigned.
///
#[derive(Clone, Debug)]
pub struct NormalizedDescriptionFragment {
    pub origin: Link,
    pub root: Root,
}

fn root_from_text(contents: &str, context: &BindingPoint) -> Result<Root, FileFormatError> {
    let fragment = HierarchyFragment::parse(contents)?;
    produce_root(&fragment, context)
}

fn load_single_fragment(
    link: &Link,
    binding: &BindingPoint,
    context: &ResolutionContext,
) -> Result<NormalizedDescriptionFragment, LoadError> {
    let origin = link.clone();
    let LoadResult { text, actual } = load_text(link, context)?;
    match root_from_text(&text, binding) {
        Ok(mut root) => {
            annotate_origins(&mut root, &actual.to_string());
            Ok(NormalizedDescriptionFragment { origin, root })
        }
        Err(inner) => Err(LoadError::FileFormatError { origin, inner }),
    }
}

fn void_take_from(mut fragment: NormalizedDescriptionFragment) -> NormalizedDescriptionFragment {
    fragment.root.take_from = vec![];
    for domain in &mut fragment.root.domains {
        domain.take_from = vec![];
        for component in &mut domain.components {
            component.take_from = vec![];
        }
    }
    fragment
}

fn fetch_connected_fragments_aux(
    fragment: NormalizedDescriptionFragment,
    visited: &mut BTreeSet<Link>,
    context: &ResolutionContext,
) -> Result<Vec<NormalizedDescriptionFragment>, LoadError> {
    let mut results = vec![];
    let NormalizedDescriptionFragment { origin, root } = &fragment;

    let visit =
        |link, binding: &BindingPoint, visited: &mut BTreeSet<Link>| -> Result<_, LoadError> {
            let new_fragment = load_single_fragment(&link, binding, context)?;
            let addend = fetch_connected_fragments_aux(new_fragment, visited, context)?;
            visited.insert(link.clone());
            Ok(addend)
        };

    visited.insert(origin.clone());

    for raw_link in &fragment.root.take_from {
        let link = Link::parse(raw_link)?;
        if visited.contains(&link) {
            return Err(LoadError::CircularDependency {
                trigger: origin.clone(),
                visited: link.clone(),
            });
        } else {
            results.extend(visit(link, &BindingPoint::Root, visited)?)
        }
    }

    for domain in &root.domains {
        let domain_binding = BindingPoint::for_domain(domain);

        for raw_link in &domain.take_from {
            let link = Link::parse(raw_link)?;
            if visited.contains(&link) {
                return Err(LoadError::CircularDependency {
                    trigger: origin.clone(),
                    visited: link.clone(),
                });
            } else {
                results.extend(visit(link, &domain_binding, visited)?)
            }
        }
        for component in &domain.components {
            let component_binding = BindingPoint::for_component(domain, component);

            for raw_link in &component.take_from {
                let link = Link::parse(raw_link)?;
                if visited.contains(&link) {
                    return Err(LoadError::CircularDependency {
                        trigger: origin.clone(),
                        visited: link.clone(),
                    });
                } else {
                    results.extend(visit(link, &component_binding, visited)?)
                }
            }
        }
    }

    results.push(void_take_from(fragment));
    Ok(results)
}

pub fn load_connected_fragments(
    fragment: NormalizedDescriptionFragment,
    context: &ResolutionContext,
) -> Result<Vec<NormalizedDescriptionFragment>, LoadError> {
    let result = fetch_connected_fragments_aux(fragment, &mut BTreeSet::new(), context)?;
    Ok(result)
}

pub fn load_fragments(
    link: Link,
    context: &ResolutionContext,
) -> Result<Vec<NormalizedDescriptionFragment>, LoadError> {
    let root_fragment = load_single_fragment(&link, &BindingPoint::Root, context)?;
    load_connected_fragments(root_fragment, context).map_err(|inner| LoadError::TakeFrom {
        address: link.clone(),
        inner: Box::new(inner),
    })
}

pub fn load_fragments_multiple_sources(
    links: impl Iterator<Item = Link>,
    context: &ResolutionContext,
) -> Result<Vec<NormalizedDescriptionFragment>, LoadError> {
    let mut collection = vec![];
    for fragment in links.map(|link| load_fragments(link, context)) {
        collection.extend(fragment?);
    }
    Ok(collection)
}

static EMBEDDED_DESCRIPTIONS_DIR: include_dir::Dir =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/../../descriptions");
