use zksync_error_model::inner::{component, domain};

use crate::description::{self, HierarchyFragmentKind};

#[derive(Clone, Debug)]
pub enum BindingPoint {
    Root,
    Domain {
        domain_binding: domain::PartialIdentifier,
    },
    Component {
        domain_binding: domain::PartialIdentifier,
        component_binding: component::PartialIdentifier,
    },
}

impl From<&BindingPoint> for HierarchyFragmentKind {
    fn from(val: &BindingPoint) -> Self {
        match val {
            BindingPoint::Root => HierarchyFragmentKind::Root,
            BindingPoint::Domain { .. } => HierarchyFragmentKind::Domain,
            BindingPoint::Component { .. } => HierarchyFragmentKind::Component,
        }
    }
}

impl BindingPoint {
    pub fn for_domain(domain: &description::Domain) -> Self {
        Self::Domain {
            domain_binding: domain.get_partial_identifier(),
        }
    }

    pub fn for_component(domain: &description::Domain, component: &description::Component) -> Self {
        Self::Component {
            domain_binding: domain.get_partial_identifier(),
            component_binding: component.get_partial_identifier(),
        }
    }
}
