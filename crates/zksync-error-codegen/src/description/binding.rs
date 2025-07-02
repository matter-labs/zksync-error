//! Binding point definitions for error hierarchy.
//!
//! This module defines the concept of "binding points" in the error description
//! hierarchy. A binding point represents a location in the hierarchy where
//! dependencies can be attached and resolved. This is used during the dependency
//! resolution process to track where each dependency originated from.

use zksync_error_model::inner::{component, domain};

use crate::description::{self, HierarchyFragmentKind};

/// Represents a point in the error hierarchy where dependencies can be bound.
///
/// Each binding point corresponds to a level in the error description hierarchy:
/// - Root level (top-level dependencies)
/// - Domain level (domain-specific dependencies)
/// - Component level (component-specific dependencies)
///
/// Binding points are used during dependency resolution to track the origin
/// of each dependency and ensure proper scoping of bindings.
#[derive(Clone, Debug)]
pub enum BindingPoint {
    /// Root-level binding point.
    ///
    /// Dependencies bound at this level are available globally across
    /// all domains and components.
    Root,

    /// Domain-level binding point.
    ///
    /// Dependencies bound at this level are scoped to a specific domain
    /// and are available to all components within that domain.
    Domain {
        /// The partial identifier of the domain this binding point represents
        domain_binding: domain::PartialIdentifier,
    },

    /// Component-level binding point.
    ///
    /// Dependencies bound at this level are scoped to a specific component
    /// within a specific domain.
    Component {
        /// The partial identifier of the domain containing this component
        domain_binding: domain::PartialIdentifier,
        /// The partial identifier of the component this binding point represents
        component_binding: component::PartialIdentifier,
    },
}

impl From<&BindingPoint> for HierarchyFragmentKind {
    /// Converts a binding point to its corresponding hierarchy fragment kind.
    ///
    /// This is used to determine the type of hierarchy fragment that corresponds
    /// to a given binding point, which is useful for validation and processing.
    fn from(val: &BindingPoint) -> Self {
        match val {
            BindingPoint::Root => HierarchyFragmentKind::Root,
            BindingPoint::Domain { .. } => HierarchyFragmentKind::Domain,
            BindingPoint::Component { .. } => HierarchyFragmentKind::Component,
        }
    }
}

impl BindingPoint {
    /// Creates a domain-level binding point.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain to create a binding point for
    ///
    /// # Returns
    ///
    /// A `BindingPoint::Domain` that represents the binding point for the given domain.
    pub fn for_domain(domain: &description::Domain) -> Self {
        Self::Domain {
            domain_binding: domain.get_partial_identifier(),
        }
    }

    /// Creates a component-level binding point.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain containing the component
    /// * `component` - The component to create a binding point for
    ///
    /// # Returns
    ///
    /// A `BindingPoint::Component` that represents the binding point for the
    /// given component within the specified domain.
    pub fn for_component(domain: &description::Domain, component: &description::Component) -> Self {
        Self::Component {
            domain_binding: domain.get_partial_identifier(),
            component_binding: component.get_partial_identifier(),
        }
    }
}
