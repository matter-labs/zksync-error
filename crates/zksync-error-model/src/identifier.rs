use crate::inner::{ErrorCode, ErrorDescription};

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
pub struct ErrorIdentifier {
    pub domain: String,
    pub component: String,
    pub code: ErrorCode,
}
impl ErrorIdentifier {
    fn identifier_builder(domain: &str, component: &str, error: &ErrorCode) -> String {
        format!("[{domain}-{component}-{error}]")
    }
}

impl std::fmt::Display for ErrorIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&Self::identifier_builder(
            &self.domain,
            &self.component,
            &self.code,
        ))
    }
}

impl ErrorDescription {
    pub fn get_identifier(&self) -> ErrorIdentifier {
        ErrorIdentifier {
            domain: self.domain.identifier.clone(),
            component: self.component.identifier.clone(),
            code: self.code,
        }
    }
}
