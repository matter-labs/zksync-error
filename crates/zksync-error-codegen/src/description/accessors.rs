use super::{Collection, Component, Domain, Error, Root};

impl Root {
    pub fn get_domain(&self, domain: &str) -> Option<&Domain> {
        self.domains.iter().find(|d| d.domain_name == domain)
    }

    pub fn get_component(&self, domain: &str, component: &str) -> Option<&Component> {
        let domain = self.get_domain(domain)?;
        let component = domain
            .components
            .iter()
            .find(|c| c.component_name == component)?;

        Some(component)
    }
}

impl Domain {
    pub fn get_component(&self, component: &str) -> Option<&Component> {
        self.components
            .iter()
            .find(|c| c.component_name == component)
    }
}

impl Collection {
    pub fn get_component(&self, domain: &str, component_name: &str) -> Option<&Component> {
        match self {
            Collection::Root(root) => root.get_component(domain, component_name),
            Collection::Domain(domain) => domain.get_component(component_name),
            Collection::Component(component) if component.component_name == component_name => {
                Some(component)
            }
            _ => None,
        }
    }
    pub fn get_domain(&self, domain_name: &str) -> Option<&Domain> {
        match self {
            Collection::Root(root) => root.get_domain(domain_name),
            Collection::Domain(domain) if domain.domain_name == domain_name => Some(domain),
            _ => None,
        }
    }
    pub fn get_component_errors(&self, domain: &str, component_name: &str) -> Option<&Vec<Error>> {
        match self {
            Collection::Root(root) => root
                .get_component(domain, component_name)
                .map(|c| &c.errors),
            Collection::Domain(domain) => domain.get_component(component_name).map(|c| &c.errors),
            Collection::Component(component) if component.component_name == component_name => {
                Some(&component.errors)
            }
            Collection::Errors(errors) => Some(errors),
            _ => None,
        }
    }
}
