use std::collections::BTreeSet;

use error::LoadError;
use fetch::LoadResult;
use fetch::load_text;
use resolution::context::ResolutionContext;
use zksync_error_model::link::Link;

use crate::description::HierarchyFragment;
use crate::description::Root;
use crate::description::accessors::annotate_origins;
use crate::description::binding::BindingPoint;
use crate::description::error::FileFormatError;
use crate::description::normalization::produce_root;
use crate::description::parsers::link;

pub mod builder;
pub mod dependency_lock;
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

impl NormalizedDescriptionFragment {
    fn void_dependencies(mut self) -> NormalizedDescriptionFragment {
        self.root = self.root.void_dependencies();
        self
    }
}

fn root_from_text(contents: &str, context: &BindingPoint) -> Result<Root, FileFormatError> {
    let fragment = HierarchyFragment::parse(contents)?;
    produce_root(&fragment, context)
}

fn load_single_fragment(
    link: &Link,
    binding: &BindingPoint,
    context: &mut ResolutionContext,
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

pub fn load_dependent_component(
    link: Link,
    context: &mut ResolutionContext,
) -> Result<Vec<NormalizedDescriptionFragment>, LoadError> {
    fn load_connected_fragments_aux(
        fragment: NormalizedDescriptionFragment,
        visited: &mut BTreeSet<Link>,
        context: &mut ResolutionContext,
    ) -> Result<Vec<NormalizedDescriptionFragment>, LoadError> {
        let mut results = vec![];
        let NormalizedDescriptionFragment { origin, root } = &fragment;

        visited.insert(origin.clone());

        for (dependency, binding) in &root.dependencies() {
            let dependency = link::parse(dependency)?;
            if !visited.insert(dependency.clone()) {
                return Err(LoadError::CircularDependency {
                    trigger: origin.clone(),
                    visited: dependency,
                });
            } else {
                let new_fragment = load_single_fragment(&dependency, binding, context)?;
                let addend = load_connected_fragments_aux(new_fragment, visited, context)?;
                results.extend(addend);
            }
        }

        results.push(fragment.void_dependencies());
        Ok(results)
    }

    let root_fragment = load_single_fragment(&link, &BindingPoint::Root, context)?;

    load_connected_fragments_aux(root_fragment, &mut BTreeSet::new(), context)
}

pub fn load_fragments_multiple_sources(
    links: impl Iterator<Item = Link>,
    context: &mut ResolutionContext,
) -> Result<Vec<NormalizedDescriptionFragment>, LoadError> {
    let mut collection = vec![];
    for link in links {
        let fragments = load_dependent_component(link, context)?;
        collection.extend(fragments);
    }
    Ok(collection)
}

static EMBEDDED_DESCRIPTIONS_DIR: include_dir::Dir =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/../../descriptions");
