pub mod error;

use error::MergeError;
use std::collections::BTreeMap;

use super::{ArrayMultilineString, Component, Domain, Root, Type};

pub trait Mergeable {
    fn merge(self, other: Self) -> Result<Self, MergeError>
    where
        Self: Sized;
}

impl<K, V> Mergeable for BTreeMap<K, V>
where
    K: Ord,
    V: Mergeable,
{
    fn merge(mut self, other: Self) -> Result<Self, MergeError>
    where
        Self: Sized,
    {
        for (key, value) in other {
            let updated = match self.remove(&key) {
                Some(existing_value) => existing_value.merge(value)?,
                None => value,
            };
            self.insert(key, updated);
        }
        Ok(self)
    }
}

impl<T> Mergeable for Option<T>
where
    T: Mergeable + Clone,
{
    fn merge(self, other: Self) -> Result<Self, MergeError> {
        match (self, other) {
            (None, None) => Ok(None),
            (fst @ Some(_), None) => Ok(fst),
            (None, other @ Some(_)) => Ok(other),
            (Some(x), Some(y)) => Ok(Some(x.merge(y)?)),
        }
    }
}
impl Mergeable for String {
    fn merge(self, other: String) -> Result<Self, MergeError> {
        if *self == other {
            Ok(self)
        } else if self.is_empty() {
            Ok(other)
        } else if other.is_empty() {
            Ok(self)
        } else {
            Err(MergeError::StringsDiffer(self.clone(), other.clone()))
        }
    }
}

impl Mergeable for ArrayMultilineString {
    fn merge(self, other: ArrayMultilineString) -> Result<Self, MergeError> {
        if self == other {
            Ok(self)
        } else if self.is_empty() {
            Ok(other)
        } else if other.is_empty() {
            Ok(self)
        } else {
            Err(MergeError::StringsDiffer(self.into(), other.into()))
        }
    }
}

impl Mergeable for Root {
    fn merge(self, other: Self) -> Result<Self, MergeError> {
        let Root {
            types: types1,
            domains: domains1,
        } = self;
        let Root {
            types: types2,
            domains: domains2,
        } = other;
        let domain_map1: BTreeMap<_, Domain> = domains1
            .into_iter()
            .map(|d| (d.get_partial_identifier(), d))
            .collect();
        let domain_map2: BTreeMap<_, Domain> = domains2
            .into_iter()
            .map(|d| (d.get_partial_identifier(), d))
            .collect();
        let merged_domains = domain_map1.merge(domain_map2)?.into_values().collect();

        let types_map1: BTreeMap<String, Type> =
            types1.into_iter().map(|t| (t.name.to_owned(), t)).collect();
        let types_map2: BTreeMap<String, Type> =
            types2.into_iter().map(|t| (t.name.to_owned(), t)).collect();
        let merged_types = types_map1.merge(types_map2)?.into_values().collect();

        Ok(Root {
            types: merged_types,
            domains: merged_domains,
        })
    }
}

impl Mergeable for Domain {
    fn merge(self, other: Self) -> Result<Self, MergeError>
    where
        Self: Sized,
    {
        if self.get_partial_identifier() == other.get_partial_identifier() {
            let component_map1: BTreeMap<_, Component> = self
                .components
                .into_iter()
                .map(|d| (d.get_partial_identifier(), d))
                .collect();
            let component_map2: BTreeMap<_, Component> = other
                .components
                .into_iter()
                .map(|d| (d.get_partial_identifier(), d))
                .collect();
            let components = component_map1
                .merge(component_map2)?
                .into_values()
                .collect();

            Ok(Self {
                domain_name: self.domain_name,
                domain_code: self.domain_code,
                identifier_encoding: self.identifier_encoding.merge(other.identifier_encoding)?,
                description: self.description.merge(other.description)?,
                components,
                bindings: self.bindings.merge(other.bindings)?,
                take_from: vec![],
                origins: [self.origins, other.origins].concat(),
            })
        } else {
            Err(MergeError::ConflictingDomainDefinitions(
                Box::new(self),
                Box::new(other),
            ))
        }
    }
}
impl Mergeable for Component {
    fn merge(self, other: Self) -> Result<Self, MergeError>
    where
        Self: Sized,
    {
        if self.get_partial_identifier() == other.get_partial_identifier() {
            Ok(Self {
                origins: [self.origins, other.origins].concat(),
                component_name: self.component_name,
                component_code: self.component_code,
                identifier_encoding: self.identifier_encoding.merge(other.identifier_encoding)?,
                description: self.description.merge(other.description)?,
                bindings: self.bindings.merge(other.bindings)?,
                take_from: vec![],
                errors: [self.errors, other.errors].concat(),
            })
        } else {
            Err(MergeError::ConflictingComponentDefinitions(
                Box::new(self),
                Box::new(other),
            ))
        }
    }
}

impl Mergeable for super::Error {
    fn merge(self, other: Self) -> Result<Self, MergeError>
    where
        Self: Sized,
    {
        if self.code == other.code && self.name == other.name && self.fields == other.fields {
            Ok(Self {
                name: self.name,
                code: self.code,
                message: self.message.merge(other.message)?,
                fields: self.fields,
                bindings: self.bindings.merge(other.bindings)?,
                doc: self.doc.merge(other.doc)?,
                origins: [self.origins, other.origins].concat(),
            })
        } else {
            Err(MergeError::ConflictingErrorDescriptions(self, other))
        }
    }
}
impl Mergeable for super::ErrorDocumentation {
    fn merge(self, other: Self) -> Result<Self, MergeError>
    where
        Self: Sized,
    {
        let description = self.description.merge(other.description)?;
        let summary = self.summary.merge(other.summary)?;
        // FIXME: probably need finer approach than just concatenating them
        let likely_causes = [self.likely_causes, other.likely_causes].concat();

        Ok(Self {
            description,
            summary,
            likely_causes,
        })
    }
}
impl Mergeable for super::ErrorType {
    fn merge(self, other: Self) -> Result<Self, MergeError>
    where
        Self: Sized,
    {
        Ok(Self {
            name: self.name.merge(other.name)?,
        })
    }
}
impl Mergeable for Type {
    fn merge(self, other: Self) -> Result<Self, MergeError>
    where
        Self: Sized,
    {
        Ok(Self {
            name: self.name.merge(other.name)?,
            description: self.description.merge(other.description)?,
            bindings: self.bindings.merge(other.bindings)?,
        })
    }
}
impl Mergeable for super::FullyQualifiedType {
    fn merge(self, other: Self) -> Result<Self, MergeError>
    where
        Self: Sized,
    {
        Ok(Self {
            expression: self.expression.merge(other.expression)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use maplit::btreemap;

    use crate::description::merge::{error::MergeError, Mergeable as _};

    #[test]
    fn btreemap_merge_success() {
        let fst = btreemap! {
            1 => "".to_owned(),
            2 => "2".to_owned(),
            3 => "3".to_owned(),
        };

        let snd = btreemap! {
            1 => "".to_owned(),
            2 => "2".to_owned(),
            4 => "4".to_owned(),
        };
        let expected = btreemap! {
            1 => "".to_owned(),
            2 => "2".to_owned(),
            3 => "3".to_owned(),
            4 => "4".to_owned(),
        };
        let merged = fst.merge(snd).unwrap();
        assert_eq!(merged, expected);
    }
    #[test]
    fn btreemap_merge_fail() {
        let fst = btreemap! {
            1 => "".to_owned(),
            2 => "2".to_owned(),
            3 => "3".to_owned(),
        };

        let snd = btreemap! {
            1 => "".to_owned(),
            2 => "22".to_owned(),
        };
        let expected = MergeError::StringsDiffer("2".into(), "22".into());
        let res = fst.merge(snd).unwrap_err();
        assert_eq!(res, expected);
    }
}
