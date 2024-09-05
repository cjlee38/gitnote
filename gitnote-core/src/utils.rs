use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;

pub trait PathBufExt {
    fn try_to_str(&self) -> anyhow::Result<&str>;
}

impl PathBufExt for PathBuf {
    fn try_to_str(&self) -> anyhow::Result<&str> {
        let result = self.to_str()
            .with_context(|| format!("[unexpected error] Failed to resolve path {:?}", self))?;
        return Ok(result);
    }
}

/// shortcut to create a file if it does not exist
pub fn create_file_if_not_exists(
    parent_dir: &PathBuf,
    filename: &str,
    content: Option<&str>
) -> anyhow::Result<File> {
    // ensure directory exists
    if !parent_dir.exists() {
        std::fs::create_dir_all(parent_dir)?;
    }

    let file_path = parent_dir.join(filename);
    if file_path.exists() {
        return Ok(File::open(&file_path)?);
    }
    let file = File::create(&file_path)
        .inspect(|mut it| { content.and_then(|c| it.write_all(c.as_bytes()).ok()); })?;
    Ok(file)
}
