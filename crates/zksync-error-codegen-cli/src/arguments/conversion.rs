use zksync_error_codegen::arguments::BackendOutput;

use super::Arguments;

impl From<Arguments> for zksync_error_codegen::arguments::GenerationArguments {
    fn from(val: Arguments) -> Self {
        let Arguments {
            root: definitions,
            backend,
            verbose,
            output_directory,
            additional_definition_files: additional_inputs,
            backend_args,
        } = val;
        zksync_error_codegen::arguments::GenerationArguments {
            verbose,
            root_link: definitions,
            outputs: vec![BackendOutput {
                output_path: output_directory.into(),
                backend: backend.into(),
                arguments: backend_args.into_iter().collect(),
            }],
            input_links: additional_inputs,
        }
    }
}
