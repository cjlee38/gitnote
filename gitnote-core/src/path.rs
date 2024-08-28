use std::path::PathBuf;
use std::string::ToString;

use anyhow::{anyhow, Context};

use crate::libgit::find_root_path;

pub struct PathResolver {
    root: PathBuf,
    relative: PathBuf,
}

impl PathResolver {
    pub fn from_input(input_path: &String) -> anyhow::Result<Self> {
        let canonical = PathBuf::from(input_path)
            .canonicalize()
            .with_context(|| format!("cannot find specified file [{input_path}]."))?;
        let root = find_root_path()?;
        let relative = canonical.strip_prefix(&root)?.to_path_buf();

        Self::validate_path(&canonical, &root)?;
        let resolver = PathResolver { root, relative };
        Ok(resolver)
    }

    fn validate_path(canonical: &PathBuf, input: &PathBuf) -> anyhow::Result<()> {
        if !canonical.exists() || !canonical.starts_with(input) {
            return Err(anyhow!(format!(
            "specified file {canonical:?} looks like not contained in git repository of {input:?}",
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::testlib::{AnyToString, TestRepo};
    use super::*;

    #[test]
    fn path_resolver() -> anyhow::Result<()> {
        let repo = TestRepo::new();
        let path = repo.create_file("foo.txt", Some("hello world"))?;
        PathResolver::from_input(&path.str())?;
        Ok(())
    }
}
