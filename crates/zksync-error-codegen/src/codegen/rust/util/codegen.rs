use crate::codegen::rust::RustBackend;
use proc_macro2::TokenStream;
use quote::quote;
use zksync_error_model::inner::Model;

pub struct DomainContext {
    pub domain: TokenStream,
    pub domain_code: TokenStream,
    pub components: Vec<TokenStream>,
    pub component_codes: Vec<TokenStream>,
}

pub struct ComponentContext {
    pub domain: TokenStream,
    pub domain_code: TokenStream,
    pub component: TokenStream,
    pub component_code: TokenStream,
}

pub fn ident(name: &str) -> TokenStream {
    sanitize(name)
        .parse()
        .unwrap_or_else(|_| panic!("Unable to parse Rust expression: {name}"))
}

/// Transform a multiline string into a documentation macro that looks as follows:
///
/// ```ignore
/// #[doc = "first line"]
/// #[doc = "second line"]
/// ...
/// ```
pub fn doc_tokens(input: &str) -> TokenStream {
    let lines = input.lines().map(|line| {
        quote! { #[doc = #line ] }
    });
    quote! { #( #lines )* }
}

pub fn sanitize(s: &str) -> String {
    super::replace_non_alphanumeric(s, '_')
}

pub fn map_domains<'a, U>(
    model: &'a Model,
    mapper: impl Fn(&DomainContext) -> U,
) -> std::iter::Map<
    impl Iterator<Item = &'a zksync_error_model::inner::DomainDescription>,
    impl FnMut(&'a zksync_error_model::inner::DomainDescription) -> U,
> {
    model.domains.values().map(move |domain| {
        let domain_context = DomainContext {
            domain: RustBackend::domain_ident(&domain.meta),
            domain_code: RustBackend::domain_code_ident(&domain.meta),
            components: domain
                .components
                .values()
                .map(|component| RustBackend::component_ident(&component.meta))
                .collect(),
            component_codes: domain
                .components
                .values()
                .map(|component| RustBackend::component_code_ident(&component.meta))
                .collect(),
        };
        mapper(&domain_context)
    })
}

pub fn map_components<'a, U>(
    model: &'a Model,
    mapper: impl Fn(&ComponentContext) -> U,
) -> std::iter::Map<
    impl Iterator<Item = &'a zksync_error_model::inner::ComponentDescription>,
    impl FnMut(&'a zksync_error_model::inner::ComponentDescription) -> U,
> {
    model.components().map(move |component| {
        let component_context = ComponentContext {
            domain: RustBackend::domain_ident(&component.meta.domain),
            domain_code: RustBackend::domain_code_ident(&component.meta.domain),
            component: RustBackend::component_ident(&component.meta),
            component_code: RustBackend::component_code_ident(&component.meta),
        };
        mapper(&component_context)
    })
}
