use std::path::PathBuf;

use cargo_metadata::MetadataCommand;
use zksync_error_model::link::Link;

use super::resolution::{ResolutionContext, overrides::Remapping};

const METADATA_CATEGORY: &str = "zksync_error_codegen";

#[derive(Clone, Debug)]
pub struct CollectionFile {
    pub package: String,
    pub absolute_path: PathBuf,
}

pub fn get_resolution_context(overrides: Remapping) -> ResolutionContext {
    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to fetch cargo metadata");

    let mut context = ResolutionContext {
        files: vec![],
        overrides,
    };

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
                        context.files.push(CollectionFile {
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

pub fn link_matches(link: &Link, file: &CollectionFile) -> bool {
    if let Link::PackageLink { package, filename } = link {
        let CollectionFile {
            package: candidate_package,
            absolute_path,
        } = file;

        if package != candidate_package {
            return false;
        };
        let pathbuf = PathBuf::from(absolute_path);
        let stripped_filename = pathbuf
            .file_name()
            .unwrap_or_else(|| panic!("Error accessing file `{absolute_path:?}`."));

        stripped_filename.to_str().is_some_and(|s| s == filename)
    } else {
        false
    }
}
