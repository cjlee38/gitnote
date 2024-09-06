use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;

use crate::config::Config;

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
pub fn create_file_if_not_exists<T>(
    parent_dir: &PathBuf,
    filename: &str,
    content: Option<T>,
) -> anyhow::Result<File>
where
    T: Writeable,
{
    // ensure directory exists
    if !parent_dir.exists() {
        std::fs::create_dir_all(parent_dir)?;
    }

    // if file already exists, return it
    let file_path = parent_dir.join(filename);
    if file_path.exists() {
        return Ok(File::open(&file_path)?);
    }

    // create file with given content
    let mut file = File::create(&file_path)?;
    if let Some(content) = content {
        content.write(&mut file)?;
    }
    Ok(file)
}

/// A simple trait which writes self to a file
pub trait Writeable {
    fn write(&self, file: &mut File) -> anyhow::Result<()>;
}

impl Writeable for &str {
    fn write(&self, file: &mut File) -> anyhow::Result<()> {
        Ok(file.write_all(self.as_bytes())?)
    }
}

impl Writeable for Config {
    fn write(&self, file: &mut File) -> anyhow::Result<()> {
        Ok(serde_yaml_ng::to_writer(file, self)?)
    }
}
