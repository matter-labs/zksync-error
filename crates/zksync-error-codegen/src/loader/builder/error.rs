use crate::loader::{
    error::{LinkError, LoadError},
    link::Link,
};
use zksync_error_model::{error::ModelValidationError, merger::error::MergeError};

#[derive(Debug)]
pub struct MissingComponent {
    pub domain_name: String,
    pub component_name: String,
}

#[derive(Debug)]
pub enum TakeFromError {
    IOError(LoadError),
    ParsingError(serde_json::Error),
    MissingComponent(MissingComponent),
    ModelBuildingError(Box<ModelBuildingError>),
    MergeError(MergeError),
    LinkError(LinkError),
}

impl From<LinkError> for TakeFromError {
    fn from(v: LinkError) -> Self {
        Self::LinkError(v)
    }
}

impl From<MergeError> for TakeFromError {
    fn from(v: MergeError) -> Self {
        Self::MergeError(v)
    }
}

impl From<ModelBuildingError> for TakeFromError {
    fn from(v: ModelBuildingError) -> Self {
        Self::ModelBuildingError(Box::new(v))
    }
}

impl From<MissingComponent> for TakeFromError {
    fn from(v: MissingComponent) -> Self {
        Self::MissingComponent(v)
    }
}

impl From<LoadError> for TakeFromError {
    fn from(v: LoadError) -> Self {
        Self::IOError(v)
    }
}
impl TakeFromError {
    pub fn from_address(self, address: &str) -> ModelBuildingError {
        ModelBuildingError::TakeFrom {
            address: address.to_string(),
            inner: self,
        }
    }
}
impl From<serde_json::Error> for TakeFromError {
    fn from(v: serde_json::Error) -> Self {
        Self::ParsingError(v)
    }
}
#[derive(Debug)]
pub enum ModelBuildingError {
    TakeFrom {
        address: String,
        inner: TakeFromError,
    },
    MergeError {
        merge_error: MergeError,
        main_model_origin: Link,
        additional_model_origin: Link,
    },
    ModelValidationError(ModelValidationError),
    LoadError(LoadError),
}

impl From<LoadError> for ModelBuildingError {
    fn from(v: LoadError) -> Self {
        Self::LoadError(v)
    }
}

impl From<ModelValidationError> for ModelBuildingError {
    fn from(v: ModelValidationError) -> Self {
        Self::ModelValidationError(v)
    }
}

impl std::fmt::Display for TakeFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TakeFromError::IOError(error) => f.write_fmt(format_args!("IO error: {error}")),
            TakeFromError::MissingComponent(MissingComponent {
                domain_name,
                component_name,
            }) => f.write_fmt(format_args!(
                "Unable to find a matching component {component_name} in the domain {domain_name}."
            )),
            TakeFromError::ParsingError(error) => {
                f.write_fmt(format_args!("Unable to parse errors: {error}."))
            }
            TakeFromError::ModelBuildingError(error) => f.write_fmt(format_args!(
                "Error while building model following a `takeFrom` link: {error}"
            )),
            TakeFromError::MergeError(error) => f.write_fmt(format_args!(
                "Error while merging with the error base fetched from `takeFrom` link: {error}"
            )),
            TakeFromError::LinkError(link_error) => f.write_fmt(format_args!(
                "Error parsing link while following a `takeFrom` link: {link_error}"
            )),
        }
    }
}
impl std::fmt::Display for ModelBuildingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelBuildingError::TakeFrom { address, inner } => {
                f.write_fmt(format_args!("Failed to import a file {address}: {inner}"))
            }
            ModelBuildingError::MergeError{merge_error, main_model_origin, additional_model_origin } => f.write_fmt(format_args!(
                "Error merging models {main_model_origin} and {additional_model_origin}: {merge_error}"
            )),
            ModelBuildingError::ModelValidationError(model_validation_error) => f.write_fmt(format_args!("Error validating combined model: {model_validation_error}")),
            ModelBuildingError::LoadError(load_error) => load_error.fmt(f),
        }
    }
}
impl std::error::Error for ModelBuildingError {}
