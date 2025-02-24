#[derive(Debug, derive_more::Display, thiserror::Error)]
pub enum LinkError {
    #[display("Link `{_0}` has an invalid format.")]
    InvalidLinkFormat(String),
}
