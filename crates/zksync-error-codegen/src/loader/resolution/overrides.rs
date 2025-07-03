use std::collections::BTreeMap;

use zksync_error_model::link::Link;

use crate::description::parsers::link::{self, LinkError};

#[derive(Clone, Debug)]
pub struct Remapping {
    pub map: BTreeMap<Link, Link>,
}

impl TryFrom<&Vec<(String, String)>> for Remapping {
    type Error = LinkError;

    fn try_from(value: &Vec<(String, String)>) -> Result<Self, Self::Error> {
        let mut map: BTreeMap<Link, Link> = Default::default();

        for (fst, snd) in value {
            map.insert(link::parse_str(fst)?, link::parse_str(snd)?);
        }

        Ok(Remapping { map })
    }
}
impl Remapping {
    pub fn apply(&self, link: &Link) -> Option<&Link> {
        if let Some(overridden) = self.map.get(link) {
            eprintln!("Overriding {link} with {overridden}");
            Some(overridden)
        } else {
            None
        }
    }
}
