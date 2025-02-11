use crate::inner::{ComponentDescription, DomainDescription};

use super::error::ModelValidationError;
use super::inner::Model;

fn find_duplicate_by<I, F, B>(iter: I, f: F) -> Option<(I::Item, I::Item)>
where
    I: Iterator,
    F: Fn(&I::Item) -> B,
    B: Eq,
    I::Item: Clone + Eq,
{
    let mut results: Vec<(I::Item, B)> = Vec::new();

    for x in iter {
        let fx = f(&x);
        for (old_x, old_fx) in &results {
            if *old_fx == fx {
                return Some((old_x.clone(), x));
            }
        }
        results.push((x, fx));
    }
    None
}

///
/// Validates the combined model, ensuring invariants like uniqueness of domain or component codes.
///
pub fn validate(model: &Model) -> Result<(), ModelValidationError> {
    ensure_unique_domains(model)?;

    for domain in model.domains.values() {
        ensure_unique_components(domain)?;
        for component in domain.components.values() {
            ensure_unique_errors(component)?;
        }
    }
    Ok(())
}

fn ensure_unique_domains(model: &Model) -> Result<(), ModelValidationError> {
    if let Some((d1, d2)) = find_duplicate_by(model.domains.values(), |d| &d.meta.name)
        .or(find_duplicate_by(model.domains.values(), |d| d.meta.code))
        .or(find_duplicate_by(model.domains.values(), |d| {
            &d.meta.identifier
        }))
    {
        Err(ModelValidationError::NonUniqueDomains(
            d1.meta.as_ref().clone(),
            d2.meta.as_ref().clone(),
        ))
    } else {
        Ok(())
    }
}
fn ensure_unique_components(domain: &DomainDescription) -> Result<(), ModelValidationError> {
    if let Some((c1, c2)) = find_duplicate_by(domain.components.values(), |c| &c.meta.name)
        .or(find_duplicate_by(domain.components.values(), |d| {
            d.meta.code
        }))
        .or(find_duplicate_by(domain.components.values(), |c| {
            &c.meta.identifier
        }))
    {
        Err(ModelValidationError::NonUniqueComponents(
            c1.meta.as_ref().clone(),
            c2.meta.as_ref().clone(),
            domain.meta.as_ref().clone(),
        ))
    } else {
        Ok(())
    }
}

fn ensure_unique_errors(component: &ComponentDescription) -> Result<(), ModelValidationError> {
    if let Some((error1, error2)) = find_duplicate_by(component.errors.iter(), |e| &e.name)
        .or(find_duplicate_by(component.errors.iter(), |e| e.code))
    {
        Err(ModelValidationError::NonUniqueErrors(
            error1.clone(),
            error2.clone(),
            component.meta.as_ref().clone(),
            component.meta.domain.as_ref().clone(),
        ))
    } else {
        Ok(())
    }
}
