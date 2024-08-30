use std::path::{Path, PathBuf};
use anyhow::{anyhow, Context};

pub trait PathBufExt {
    fn try_to_str(&self) -> anyhow::Result<&str>;
    fn parent_dir(&self) -> Option<&Path>;
}

impl PathBufExt for PathBuf {
    fn try_to_str(&self) -> anyhow::Result<&str> {
        let result = self.to_str()
            .with_context(|| format!("[unexpected error] Failed to resolve path {:?}", self))?;
        return Ok(result);
    }

    fn parent_dir(&self) -> Option<&Path> {
        let parent = self.parent()?;
        if parent == Path::new("") {
            return Some(Path::new("."));
        }
        return Some(parent);
    }
}
