use cargo_metadata::MetadataCommand;

use super::{DescriptionFile, ResolutionContext};

const METADATA_CATEGORY: &str = "zksync_error_codegen";

pub fn get_resolution_context() -> ResolutionContext {
    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to fetch cargo metadata");

    let mut context = ResolutionContext::default();

    for pkg in &metadata.packages {
        if let Some(codegen_meta) = pkg.metadata.get(METADATA_CATEGORY) {
            if let Some(json_files) = codegen_meta.get("json_files").and_then(|x| x.as_array()) {
                for path_value in json_files {
                    if let Some(rel_path) = path_value.as_str() {
                        let package_root = pkg
                            .manifest_path
                            .parent() // removing Cargo.toml
                            .unwrap();
                        let absolute_path = package_root.join(rel_path).into();
                        context.files.push(DescriptionFile {
                            package: pkg.name.to_owned(),
                            absolute_path,
                        });
                    }
                }
            }
        }
    }

    context
}
