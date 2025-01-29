#[derive(Debug)]
pub enum MergeError {
    DuplicateTypeBinding(String),
    ConflictingTypeDescriptions(String),
    ConflictingDomainDefinitions(String),
    StringsDiffer(String, String),
    ConflictingComponentDefinitions(String),
    ConflictingErrorDescriptions(String, String),
}
impl std::fmt::Display for MergeError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for MergeError {}
