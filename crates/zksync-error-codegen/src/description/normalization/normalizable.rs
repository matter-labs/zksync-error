use zksync_error_model::inner::component;
use zksync_error_model::inner::domain;

use crate::description::Component;
use crate::description::Domain;
use crate::description::Root;

///
/// Can be normalized to a full description file.
///
pub trait Normalizable {
    type Context;
    fn normalize(&self, context: &Self::Context) -> Root;
}

impl Normalizable for Component {
    type Context = domain::PartialIdentifier;

    fn normalize(&self, context: &Self::Context) -> Root {
        let domain::PartialIdentifier { name, code } = context;
        Root {
            domains: vec![Domain {
                domain_name: name.clone(),
                domain_code: *code,
                identifier_encoding: None,
                components: vec![self.clone()],
                ..Default::default()
            }],
            types: vec![],
        }
    }
}

impl Normalizable for Domain {
    type Context = ();

    fn normalize(&self, _: &Self::Context) -> Root {
        Root {
            domains: vec![self.clone()],
            types: vec![],
        }
    }
}

impl Normalizable for Vec<crate::description::Error> {
    type Context = (domain::PartialIdentifier, component::PartialIdentifier);

    fn normalize(&self, context: &Self::Context) -> Root {
        let (domain, component) = context;
        Root {
            domains: vec![Domain {
                domain_name: domain.name.clone(),
                domain_code: domain.code,
                identifier_encoding: None,
                components: vec![Component {
                    component_name: component.name.clone(),
                    component_code: component.code,
                    identifier_encoding: None,
                    errors: self.clone(),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            types: vec![],
        }
    }
}
