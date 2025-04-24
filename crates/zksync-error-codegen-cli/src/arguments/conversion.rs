use std::collections::BTreeMap;

use zksync_error_codegen::arguments::BackendOutput;

use crate::error::ApplicationError;

use super::Arguments;

impl TryFrom<Arguments> for zksync_error_codegen::arguments::GenerationArguments {
    type Error = ApplicationError;

    fn try_from(value: Arguments) -> Result<Self, Self::Error> {
        let Arguments {
            sources,
            backend,
            verbose,
            output_directory,
            backend_args,
            remap,
        } = value;

        let override_map: BTreeMap<String, String> = {
            if let Some(remap) = remap {
                serde_json::from_str(&remap).map_err(|e| ApplicationError::InvalidArgument {
                    argument: remap,
                    reason: e.to_string(),
                })?
            } else {
                Default::default()
            }
        };

        Ok(zksync_error_codegen::arguments::GenerationArguments {
            verbose,
            input_links: sources,
            override_links: override_map.into_iter().collect(),
            outputs: vec![BackendOutput {
                output_path: output_directory.into(),
                backend: backend.into(),
                arguments: backend_args.into_iter().collect(),
            }],
        })
    }
}
