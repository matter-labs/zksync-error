use std::collections::BTreeMap;

use zksync_error_codegen::arguments::{BackendOutput, ResolutionMode};

use crate::{arguments::Mode, error::ApplicationError};

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
            mode,
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

        let resolution_mode = match mode {
            Mode::NoLock => ResolutionMode::NoLock {
                override_links: override_map.into_iter().collect(),
            },
        };

        Ok(zksync_error_codegen::arguments::GenerationArguments {
            verbose,
            input_links: sources,
            mode: resolution_mode,
            outputs: vec![BackendOutput {
                output_path: output_directory.into(),
                backend: backend.into(),
                arguments: backend_args,
            }],
        })
    }
}
