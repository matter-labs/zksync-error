use crate::model;
use zksync_error_model::unpacked as inner;

impl From<inner::TargetLanguageType> for model::TargetLanguageType {
    fn from(val: inner::TargetLanguageType) -> Self {
        let inner::TargetLanguageType { name, path } = val;
        model::TargetLanguageType { name, path }
    }
}

impl From<inner::TypeMetadata> for model::TypeMetadata {
    fn from(val: inner::TypeMetadata) -> Self {
        let inner::TypeMetadata { description } = val;
        model::TypeMetadata { description }
    }
}

impl From<inner::TypeDescription> for model::TypeDescription {
    fn from(val: inner::TypeDescription) -> Self {
        let inner::TypeDescription {
            name,
            meta,
            bindings,
        } = val;
        model::TypeDescription {
            name,
            meta: meta.into(),
            bindings: bindings.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl From<inner::DomainMetadata> for model::DomainMetadata {
    fn from(val: inner::DomainMetadata) -> Self {
        let inner::DomainMetadata {
            name,
            code,
            components,
            bindings,
            identifier,
            description,
        } = val;
        model::DomainMetadata {
            name,
            code,
            components,
            bindings,
            identifier,
            description,
        }
    }
}

impl From<inner::UnpackedModel> for model::ErrorHierarchy {
    fn from(val: inner::UnpackedModel) -> Self {
        let inner::UnpackedModel {
            types,
            domains,
            components,
            errors,
        } = val;
        model::ErrorHierarchy {
            types: types.into_iter().map(|(k, v)| (k, v.into())).collect(),
            domains: domains.into_iter().map(|(k, v)| (k, v.into())).collect(),
            components: components.into_iter().map(|(k, v)| (k, v.into())).collect(),
            errors: errors.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl From<inner::ComponentMetadata> for model::ComponentMetadata {
    fn from(val: inner::ComponentMetadata) -> Self {
        let inner::ComponentMetadata {
            name,
            code,
            domain_name,
            bindings,
            identifier,
            description,
        } = val;
        model::ComponentMetadata {
            name,
            code,
            domain_name,
            bindings,
            identifier,
            description,
        }
    }
}

impl From<inner::ErrorDescription> for model::ErrorDescription {
    fn from(val: inner::ErrorDescription) -> Self {
        let inner::ErrorDescription {
            domain,
            component,
            name,
            code,
            identifier,
            message,
            fields,
            documentation,
            bindings,
        } = val;
        model::ErrorDescription {
            domain,
            component,
            name,
            code,
            identifier,
            message,
            fields: fields.into_iter().map(|f| f.into()).collect(),
            documentation: documentation.map(|d| d.into()),
            bindings: bindings.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl From<inner::FieldDescription> for model::FieldDescription {
    fn from(val: inner::FieldDescription) -> Self {
        let inner::FieldDescription { name, r#type } = val;
        model::FieldDescription { name, r#type }
    }
}

impl From<inner::ErrorDocumentation> for model::ErrorDocumentation {
    fn from(val: inner::ErrorDocumentation) -> Self {
        let inner::ErrorDocumentation {
            description,
            summary: short_description,
            likely_causes,
        } = val;
        model::ErrorDocumentation {
            description,
            summary: short_description,
            likely_causes: likely_causes.into_iter().map(|lc| lc.into()).collect(),
        }
    }
}

impl From<inner::LikelyCause> for model::LikelyCause {
    fn from(val: inner::LikelyCause) -> Self {
        let inner::LikelyCause {
            cause,
            fixes,
            report,
            owner,
            references,
        } = val;
        model::LikelyCause {
            cause,
            fixes,
            report,
            owner: owner.clone().map(|o| o.into()),
            references,
        }
    }
}

impl From<inner::VersionedOwner> for model::VersionedOwner {
    fn from(val: inner::VersionedOwner) -> Self {
        let inner::VersionedOwner { name, version } = val;
        model::VersionedOwner { name, version }
    }
}
