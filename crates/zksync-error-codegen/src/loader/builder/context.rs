#![allow(unused)]

use std::rc::Rc;

use zksync_error_model::inner::ComponentMetadata;
use zksync_error_model::inner::DomainMetadata;
use zksync_error_model::link::Link;

pub struct ModelTranslationContext;
pub(super) struct TypeTranslationContext<'a> {
    pub type_name: &'a str,
    pub parent: &'a ModelTranslationContext,
}
pub(super) struct DomainTranslationContext<'a> {
    pub parent: &'a ModelTranslationContext,
}

pub(super) struct ComponentTranslationContext<'a> {
    pub domain: Rc<DomainMetadata>,
    pub parent: &'a DomainTranslationContext<'a>,
}

impl ComponentTranslationContext<'_> {
    pub(super) fn get_domain(&self) -> String {
        self.domain.identifier.name.to_string()
    }
}

pub(super) struct ErrorTranslationContext<'a> {
    pub component: Rc<ComponentMetadata>,
    pub parent: &'a ComponentTranslationContext<'a>,
}

impl ErrorTranslationContext<'_> {
    fn get_component(&self) -> String {
        self.component.identifier.name.to_string()
    }
    fn get_domain(&self) -> String {
        self.parent.get_domain()
    }
}
