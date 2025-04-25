use std::{
    io::Write,
    path::{Path, PathBuf},
};

use crate::backend::file::File;

pub(crate) fn create_files_in_result_directory(
    result_dir: &PathBuf,
    files: Vec<File>,
) -> std::io::Result<()> {
    let result_dir = Path::new(result_dir);

    if !result_dir.exists() {
        std::fs::create_dir(result_dir)?;
    }

    for file in files {
        let path = result_dir.join(file.relative_path);

        if let Some(parent_dir) = path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let mut output_file = std::fs::File::create(&path)?;
        output_file.write_all(file.content.as_bytes())?;
    }

    Ok(())
}
