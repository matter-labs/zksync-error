pub mod error;

use error::MergeError;
use std::collections::BTreeMap;

use super::inner::{
    ComponentDescription, DomainDescription, ErrorDescription, ErrorDocumentation, Model, TypeDescription,
};

fn merge_maps<K, V>(main: &mut BTreeMap<K, V>, other: &BTreeMap<K, V>) -> Result<(), MergeError>
where
    K: Eq + PartialEq + Ord + Clone,
    V: Merge + Clone,
{
    for (key, value) in other {
        if let Some(existing_value) = main.get(key).as_mut() {
            let mut new = existing_value.clone();
            new.merge(value)?;
            main.insert(key.clone(), new);
        } else {
            main.insert(key.clone(), value.clone());
        }
    }
    Ok(())
}

pub trait Merge {
    fn merge(&mut self, other: &Self) -> Result<(), MergeError>;
}

impl Merge for String {
    fn merge(&mut self, other: &Self) -> Result<(), MergeError> {
        if self == other {
            Ok(())
        } else if self.is_empty() {
            *self = other.clone();
            Ok(())
        } else if other.is_empty() {
            Ok(())
        } else {
            Err(MergeError::StringsDiffer(self.clone(), other.clone()))
        }
    }
}

impl Merge for super::inner::TargetLanguageType {
    fn merge(&mut self, other: &Self) -> Result<(), MergeError> {
        self.name.merge(&other.name)
    }
}

impl Merge for super::inner::FullyQualifiedTargetLanguageType {
    fn merge(&mut self, other: &Self) -> Result<(), MergeError> {
        self.name.merge(&other.name)?;
        self.path.merge(&other.path)
    }
}
impl Merge for TypeDescription {
    fn merge(&mut self, other: &TypeDescription) -> Result<(), MergeError> {
        if self.name == other.name {
            if self.meta.description.is_empty() && !other.meta.description.is_empty() {
                self.meta.description = other.meta.description.clone();
            } else if !self.meta.description.is_empty() && !other.meta.description.is_empty() {
                return Err(MergeError::ConflictingTypeDescriptions(self.name.clone()));
            }
            merge_maps(&mut self.bindings, &other.bindings)
        } else {
            Err(MergeError::ConflictingTypeDescriptions(self.name.clone()))
        }
    }
}

impl Merge for Model {
    fn merge(&mut self, other: &Model) -> Result<(), MergeError> {
        merge_maps(&mut self.types, &other.types)?;
        merge_maps(&mut self.domains, &other.domains)

    }
}

impl Merge for DomainDescription {
    fn merge(&mut self, other: &DomainDescription) -> Result<(), MergeError> {
        if self.meta.name != other.meta.name || self.meta.code != other.meta.code {
            return Err(MergeError::ConflictingDomainDefinitions(
                self.meta.name.clone(),
            ));
        }
        // merge_maps(&mut self.meta.bindings, &other.meta.bindings)?;
        // if !other.meta.identifier.is_empty() {
        //     self.meta.identifier = other.meta.identifier.clone();
        // }
        // if !other.meta.description.is_empty() {
        //     self.meta.description = other.meta.description.clone();
        // }
        merge_maps(&mut self.components, &other.components)
    }
}

impl Merge for ComponentDescription {
    fn merge(&mut self, other: &ComponentDescription) -> Result<(), MergeError> {
        if self.meta.name != other.meta.name || self.meta.code != other.meta.code {
            return Err(MergeError::ConflictingComponentDefinitions(
                self.meta.name.clone(),
            ));
        }
        // merge_maps(&mut self.meta.bindings, &other.meta.bindings)?;
        for error in &other.errors {
            if let Some(existing_error) = self.errors.iter_mut().find(|e| e.code == error.code) {
                existing_error.merge(error)?;
            } else {
                self.errors.push(error.clone());
            }
        }
        Ok(())
    }
}

impl Merge for ErrorDescription {
    fn merge(&mut self, other: &ErrorDescription) -> Result<(), MergeError> {
        if self.name != other.name
            || self.code != other.code
            || self.domain != other.domain
            || self.component != other.component
            || self.fields != other.fields
        {
            return Err(MergeError::ConflictingErrorDescriptions(
                self.name.clone(),
                other.name.clone(),
            ));
        }
        let _ = self.documentation.merge(&other.documentation);
        let _ = self.message.merge(&other.message);
        merge_maps(&mut self.bindings, &other.bindings)
    }
}

impl Merge for ErrorDocumentation {
    fn merge(&mut self, other: &ErrorDocumentation) -> Result<(), MergeError> {
        let _ = self.description.merge(&other.description);
        let _ = self.summary.merge(&other.summary);
        //FIXME: merge likely causes properly too
        self.likely_causes
            .extend(other.likely_causes.iter().cloned());
        Ok(())
    }
}

impl<T> Merge for Option<T>
where
    T: Merge + Clone,
{
    fn merge(&mut self, other: &Self) -> Result<(), MergeError> {
        match (self.as_mut(), other) {
            (None, None) => Ok(()),
            (Some(_), None) => todo!(),
            (None, Some(_)) => {
                *self = other.clone();
                Ok(())
            }
            (Some(x), Some(y)) => x.merge(y),
        }
    }
}
