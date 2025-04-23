use std::collections::BTreeSet;

use error::LoadError;
use error::TakeFromError;
use fetch::load_text;
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
) -> Result<NormalizedDescriptionFragment, LoadError> {
    let origin = link.clone();
    let contents = load_text(link)?;
    match root_from_text(&contents, binding) {
        Ok(mut root) => {
            annotate_origins(&mut root, &origin.to_string());

            Ok(NormalizedDescriptionFragment { origin, root })
        }
        Err(inner) => Err(LoadError::FileFormatError { origin, inner }),
    }
}

fn fetch_connected_fragments_aux(
    fragment: NormalizedDescriptionFragment,
    visited: &mut BTreeSet<Link>,
) -> Result<Vec<NormalizedDescriptionFragment>, TakeFromError> {
    let mut results = vec![];
    let NormalizedDescriptionFragment { origin, root } = &fragment;

    let visit =
        |link, binding: &BindingPoint, visited: &mut BTreeSet<Link>| -> Result<_, TakeFromError> {
            let new_fragment = load_single_fragment(&link, binding)?;
            let addend = fetch_connected_fragments_aux(new_fragment, visited)?;
            visited.insert(link.clone());
            Ok(addend)
        };

    visited.insert(origin.clone());

    for domain in &root.domains {
        let domain_binding = BindingPoint::for_domain(domain);

        for raw_link in &domain.take_from {
            let link = Link::parse(raw_link)?;
            if visited.contains(&link) {
                return Err(TakeFromError::CircularDependency {
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
                    return Err(TakeFromError::CircularDependency {
                        trigger: origin.clone(),
                        visited: link.clone(),
                    });
                } else {
                    results.extend(visit(link, &component_binding, visited)?)
                }
            }
        }
    }
    results.push(fragment);
    Ok(results)
}

pub fn load_connected_fragments(
    fragment: NormalizedDescriptionFragment,
) -> Result<Vec<NormalizedDescriptionFragment>, TakeFromError> {
    let result = fetch_connected_fragments_aux(fragment, &mut BTreeSet::new())?;
    Ok(result)
}

pub fn load_fragments(link: Link) -> Result<Vec<NormalizedDescriptionFragment>, TakeFromError> {
    let root_fragment = load_single_fragment(&link, &BindingPoint::Root)?;
    load_connected_fragments(root_fragment)
}

pub(crate) static ZKSYNC_ROOT_CONTENTS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../zksync-root.json"
));
