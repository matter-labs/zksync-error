use crate::inner::{ErrorCode, ErrorDescription};

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct PublicErrorIdentifier {
    pub domain: String,
    pub component: String,
    pub code: ErrorCode,
}
impl PublicErrorIdentifier {
    fn identifier_builder(domain: &str, component: &str, error: &ErrorCode) -> String {
        format!("[{domain}-{component}-{error}]")
    }
}

impl std::fmt::Display for PublicErrorIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&Self::identifier_builder(
            &self.domain,
            &self.component,
            &self.code,
        ))
    }
}

impl ErrorDescription {
    pub fn get_identifier(&self) -> PublicErrorIdentifier {
        PublicErrorIdentifier {
            domain: self.domain.identifier.encoding.clone(),
            component: self.component.identifier.encoding.clone(),
            code: self.code,
        }
    }
}
