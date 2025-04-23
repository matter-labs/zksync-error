use zksync_error_model::inner::{component, domain};

use crate::description::Component;
use crate::description::Domain;
use crate::description::Root;
use crate::description::error::FileFormatError;
use crate::util::LooseEq;

pub fn annotate_origins(root: &mut Root, origin: &str) {
    let annotation = origin.to_owned();
    for domain in &mut root.domains {
        domain.origins.push(annotation.clone());
        for component in &mut domain.components {
            component.origins.push(annotation.clone());
            for error in &mut component.errors {
                error.origins.push(annotation.clone());
            }
        }
    }
}

impl Root {
    pub fn get_domains_loosely_matching(&self, domain: &domain::PartialIdentifier) -> Vec<&Domain> {
        self.domains
            .iter()
            .filter(|d| d.loosely_match(domain))
            .collect()
    }

    pub fn select_domain(
        &self,
        query: &domain::PartialIdentifier,
    ) -> Result<&Domain, FileFormatError> {
        let expected = query.clone();
        let found_domains = self.get_domains_loosely_matching(query);
        match found_domains.len() {
            0 => Err(FileFormatError::NoDomains { expected }),
            1 => {
                let domain = *found_domains.first().unwrap();
                if &Into::<domain::PartialIdentifier>::into(domain) == query {
                    Ok(domain)
                } else {
                    Err(FileFormatError::NoDomains { expected })
                }
            }
            _ => {
                let domains = found_domains.into_iter().cloned().collect();
                Err(FileFormatError::MultipleDomains { expected, domains })
            }
        }
    }
}

impl Domain {
    pub fn get_components_loosely_matching(
        &self,
        component: &component::PartialIdentifier,
    ) -> Vec<&Component> {
        self.components
            .iter()
            .filter(|c| c.loosely_match(component))
            .collect()
    }

    pub fn select_component(
        &self,
        component: &component::PartialIdentifier,
    ) -> Result<&Component, FileFormatError> {
        let expected = component.clone();

        let found_components = self.get_components_loosely_matching(component);
        match found_components.len() {
            0 => Err(FileFormatError::NoComponents { expected }),
            1 => Ok(found_components.first().unwrap()),
            _ => {
                let components = found_components.into_iter().cloned().collect();
                Err(FileFormatError::MultipleComponents {
                    expected,
                    components,
                })
            }
        }
    }
}
