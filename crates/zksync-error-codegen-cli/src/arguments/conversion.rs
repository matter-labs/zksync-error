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
            lock_file,
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

        const DEFAULT_LOCK_FILE_NAME: &str = "zksync-error.lock";
        let resolution_mode = match mode {
            Mode::NoLock => ResolutionMode::NoLock {
                override_links: override_map.into_iter().collect(),
            },
            Mode::Normal => ResolutionMode::Normal {
                override_links: override_map.into_iter().collect(),
                lock_file: lock_file.unwrap_or(DEFAULT_LOCK_FILE_NAME.to_owned()),
            },
            Mode::Reproducible => ResolutionMode::Reproducible {
                lock_file: lock_file.unwrap_or(DEFAULT_LOCK_FILE_NAME.to_owned()),
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
