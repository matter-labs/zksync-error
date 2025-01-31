#![allow(unused)]

use std::rc::Rc;

use zksync_error_model::inner::ComponentMetadata;
use zksync_error_model::inner::DomainMetadata;

use crate::loader::link::Link;

pub struct ModelTranslationContext {
    pub origin: Link,
}
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
        self.domain.name.to_string()
    }
}

pub(super) struct ErrorTranslationContext<'a> {
    pub component: Rc<ComponentMetadata>,
    pub parent: &'a ComponentTranslationContext<'a>,
}

impl ErrorTranslationContext<'_> {
    fn get_component(&self) -> String {
        self.component.name.to_string()
    }
    fn get_domain(&self) -> String {
        self.parent.get_domain()
    }
}
