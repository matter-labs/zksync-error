use std::collections::BTreeMap;

use zksync_error_model::link::{Link, error::LinkError};

#[derive(Clone, Debug)]
pub struct Remapping {
    pub map: BTreeMap<Link, Link>,
}

impl TryFrom<&Vec<(String, String)>> for Remapping {
    type Error = LinkError;

    fn try_from(value: &Vec<(String, String)>) -> Result<Self, Self::Error> {
        let mut map: BTreeMap<Link, Link> = Default::default();

        for (fst, snd) in value {
            map.insert(Link::parse(fst)?, Link::parse(snd)?);
        }

        Ok(Remapping { map })
    }
}
