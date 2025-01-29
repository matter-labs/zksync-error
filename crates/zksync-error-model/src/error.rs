#[derive(Debug)]
pub enum ModelValidationError {
    UnknownType(String),
    UnmappedType(String),
    UnmappedName(String),
}
impl std::fmt::Display for ModelValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelValidationError::UnknownType(type_name) => f.write_fmt(format_args!("Unknown model type {type_name}. Ensure the \"types\" object of the error definitions file contains it.")),
            ModelValidationError::UnmappedName(name) => f.write_fmt(format_args!("The name {name} has no mapping.")),
            ModelValidationError::UnmappedType(type_name) => f.write_fmt(format_args!("Type {type_name} has no mappings for the Rust backend.")),
            _ => f.write_fmt(format_args!("{self:#?}"))

        }
    }
}
impl std::error::Error for ModelValidationError {}
