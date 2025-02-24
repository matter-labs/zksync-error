use crate::description::{Component, Domain};
use crate::util::LooseEq;

use zksync_error_model::inner::{component, domain};

impl Domain {
    pub fn get_partial_identifier(&self) -> domain::PartialIdentifier {
        domain::PartialIdentifier {
            name: self.domain_name.clone(),
            code: self.domain_code,
        }
    }
}

impl From<&Domain> for domain::PartialIdentifier {
    fn from(val: &Domain) -> Self {
        val.get_partial_identifier()
    }
}
impl From<&mut Domain> for domain::PartialIdentifier {
    fn from(val: &mut Domain) -> Self {
        val.get_partial_identifier()
    }
}
impl Component {
    pub fn get_partial_identifier(&self) -> component::PartialIdentifier {
        component::PartialIdentifier {
            name: self.component_name.clone(),
            code: self.component_code,
        }
    }
}

impl From<&Component> for component::PartialIdentifier {
    fn from(val: &Component) -> Self {
        val.get_partial_identifier()
    }
}
impl From<&mut Component> for component::PartialIdentifier {
    fn from(val: &mut Component) -> Self {
        val.get_partial_identifier()
    }
}

impl LooseEq<domain::PartialIdentifier> for Domain {
    fn loosely_match(&self, template: &domain::PartialIdentifier) -> bool {
        self.domain_code == template.code || self.domain_name == template.name
    }
}
impl LooseEq<component::PartialIdentifier> for Component {
    fn loosely_match(&self, template: &component::PartialIdentifier) -> bool {
        self.component_code == template.code || self.component_name == template.name
    }
}
