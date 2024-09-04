use std::path::PathBuf;

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