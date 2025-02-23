pub mod binding;
pub mod normalizable;

use binding::BindingPoint;
use normalizable::Normalizable as _;
use zksync_error_model::inner::component;
use zksync_error_model::inner::domain;

use crate::description::Root;

use super::error::FileFormatError;
use super::HierarchyFragment;
use super::HierarchyFragmentKind;

///
/// Takes a hierarchy fragment and a path where it has to be merged and produces a root hierarchy ready to be merged.
/// If the fragment contained only a component, it is put in a hierarchy tree within a domain provided by the context.
/// If the fragment contained only domain, it will be put in a structure with `types` and `domains` fields, and so on.
/// Invalid combinations (e.g. putting a component where a domain is expected)
/// are rejected.
///
pub fn produce_root(
    fragment: &HierarchyFragment,
    context: &BindingPoint,
) -> Result<Root, FileFormatError> {
    println!("{fragment:?} and context: {context:?}");
    match (fragment, context) {
        (HierarchyFragment::Root(root), BindingPoint::Root) => Ok(root.clone()),
        (
            HierarchyFragment::Root(root),
            BindingPoint::Domain {
                domain_binding: domain,
            },
        ) => Ok(root.select_domain(domain)?.normalize(&())),
        (
            HierarchyFragment::Root(root),
            BindingPoint::Component {
                domain_binding: domain,
                component_binding: component,
            },
        ) => Ok(root
            .select_domain(domain)
            .and_then(|d| d.select_component(component))?
            .normalize(domain)),
        (HierarchyFragment::Domain(domain), BindingPoint::Root) => Ok(domain.normalize(&())),
        (HierarchyFragment::Domain(domain), BindingPoint::Domain { domain_binding }) => {
            if &Into::<domain::PartialIdentifier>::into(domain) == domain_binding {
                Ok(domain.normalize(&()))
            } else {
                let domain = domain.clone();
                Err(FileFormatError::WrongDomain {
                    expected: domain_binding.clone(),
                    domain,
                })
            }
        }
        (
            HierarchyFragment::Domain(domain),
            BindingPoint::Component {
                domain_binding,
                component_binding,
            },
        ) => Ok(domain
            .select_component(component_binding)?
            .normalize(domain_binding)),
        (
            HierarchyFragment::Component(component),
            BindingPoint::Component {
                domain_binding,
                component_binding,
            },
        ) => {
            if &Into::<component::PartialIdentifier>::into(component) == component_binding {
                Ok(component.normalize(domain_binding))
            } else {
                let component = component.clone();
                Err(FileFormatError::WrongComponent {
                    expected: component_binding.clone(),
                    component,
                })
            }
        }
        (
            HierarchyFragment::Errors(vec),
            BindingPoint::Component {
                domain_binding,
                component_binding,
            },
        ) => Ok(vec.normalize(&(domain_binding.clone(), component_binding.clone()))),

        (HierarchyFragment::Component(_), BindingPoint::Root)
        | (HierarchyFragment::Component(_), BindingPoint::Domain { .. })
        | (HierarchyFragment::Errors(_), BindingPoint::Domain { .. })
        | (HierarchyFragment::Errors(_), BindingPoint::Root) => {
            Err(FileFormatError::UnexpectedFormat {
                expected: vec![HierarchyFragmentKind::Domain, HierarchyFragmentKind::Root],
                got: fragment.into(),
            })
        }
    }
}
