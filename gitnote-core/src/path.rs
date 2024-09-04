use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{anyhow, Context};

const NOTE_PATH: &'static str = ".git/notes";

#[derive(Debug)]
pub struct PathResolver;

impl PathResolver {
    pub fn resolve(
        current_path: &Path,
        input: &str,
    ) -> anyhow::Result<Paths> {
        let root = Self::root_by_recursive(current_path)?;
        Self::initialize(&root)?;

        let canonical = PathBuf::from(current_path.join(input))
            .canonicalize()
            .context(format!("cannot find specified file `{input}` from {:?}.", current_path))?;
        Self::validate_path(&root, &canonical)?;
        let relative = canonical.strip_prefix(&root)?.to_path_buf();
        Ok(Paths::new(root.clone(), relative))
    }

    fn root_by_recursive(current: &Path) -> anyhow::Result<PathBuf> {
        let mut current = current;
        while !current.join(".git").exists() {
            if current.parent().is_none() {
                return Err(anyhow!("Cannot find git repository from {:?}", current));
            }
            current = current.parent().expect("Cannot find parent directory");
        }
        Ok(current.to_path_buf())
    }

    fn initialize(root: &Path) -> anyhow::Result<()> {
        let note_path = root.join(NOTE_PATH);
        ensure_dir(&note_path)?;

        let description = note_path.join("description");
        if !description.exists() {
            let mut description = File::create(&description)?;
            description.write_all("This directory contains notes by `git-note`".as_bytes())?;
        }
        Ok(())
    }

    fn validate_path(root: &PathBuf, canonical: &PathBuf) -> anyhow::Result<()> {
        if !canonical.exists() || !canonical.starts_with(root) {
            return Err(anyhow!(
                "specified file `{canonical:?}` looks like not contained in git repository of `{root:?}`",
            ));
        }
        Ok(())
    }
}

/// A path entity given by path resolver.
/// Note that canonical path with care, as the location of repository itself
/// could be changed.
///
/// e.g. When repository path is `/foo` and target file located at `/foo/bar/baz.txt`, then
/// - root : `/foo`
/// - relative : `bar/baz.txt`
/// - canonical : `/foo/bar/baz.txt`
/// - home : `/foo/.git/notes`
/// - config: `/foo/.git/notes/config.yml`
/// - note : `/foo/.git/notes/12/34567890`
#[derive(Debug, Clone)]
pub struct Paths {
    root: PathBuf,
    relative: PathBuf,
}

impl Paths {
    pub fn new(root: PathBuf, relative: PathBuf) -> Self {
        Paths { root, relative }
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn relative(&self) -> &PathBuf {
        &self.relative
    }

    pub fn canonical(&self) -> PathBuf {
        self.root.join(&self.relative)
    }

    pub fn home(&self) -> PathBuf {
        static NOTE_PATH: &'static str = ".git/notes";

        self.root.join(NOTE_PATH)
    }

    pub fn config(&self) -> PathBuf {
        self.home().join("config")
    }

    pub fn note(&self, id: &String) -> anyhow::Result<PathBuf> {
        let home = self.home();
        let dir = &id[0..2];
        let file = &id[2..];
        ensure_dir(&home.join(dir))?;
        let note_path = home.join(dir).join(file);
        Ok(note_path)
    }
}

impl Display for Paths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical().display())
    }
}

fn ensure_dir(dir_path: &PathBuf) -> anyhow::Result<()> {
    if !dir_path.exists() {
        return Ok(fs::create_dir_all(dir_path).context(format!(
            "Failed to create directory at path: {:?}",
            dir_path
        ))?);
    }
    return Ok(());
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::path::PathResolver;
    use crate::testlib::TestRepo;

    #[test]
    pub fn resolve() -> anyhow::Result<()> {
        // given
        let repo = TestRepo::new();
        let path = repo.create_file("foo.txt", Some("hello world"))?;

        // when
        let paths = PathResolver::resolve(repo.path(), "foo.txt")?;

        // then
        assert_eq!(paths.root(), repo.path());
        assert_eq!(paths.canonical(), path);
        assert_eq!(paths.home(), repo.path().join(".git/notes"));
        assert_eq!(paths.relative(), &PathBuf::from("foo.txt"));
        Ok(())
    }

    #[test]
    pub fn resolve_in_nested() -> anyhow::Result<()> {
        // given
        let repo = TestRepo::new();
        repo.create_dir("foo")?;
        repo.create_dir("foo/bar")?;
        let path = repo.create_file("foo/bar/baz.txt", Some("hello world"))?;

        // when
        let paths = PathResolver::resolve(&repo.path().join("foo"), "./bar/baz.txt")?;
        assert_eq!(paths.root(), repo.path());
        assert_eq!(paths.canonical(), path);
        assert_eq!(paths.home(), repo.path().join(".git/notes"));
        assert_eq!(paths.relative(), &PathBuf::from("foo/bar/baz.txt"));
        Ok(())
    }
}
