use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};

use crate::libgit::execute_git_command;

const NOTE_PATH: &'static str = ".git/notes";

#[derive(Debug)]
pub struct PathResolver {
    root: PathBuf,
}

impl PathResolver {
    pub fn from_input(current_path: &Path) -> anyhow::Result<Self> {
        let root = execute_git_command(current_path, vec!["rev-parse", "--show-toplevel"])?
            .parse::<PathBuf>()
            .map_err(|e| anyhow!("Failed to parse path: {}", e))?;
        let resolver = PathResolver { root };
        resolver.initialize()?;
        Ok(resolver)
    }

    fn initialize(&self) -> anyhow::Result<()> {
        let note_path = self.root.join(NOTE_PATH);
        ensure_dir(&note_path)?;

        let description = note_path.join("description");
        if !Path::new(&description).exists() {
            let mut description = File::create(&description)?;
            description.write_all("This directory contains notes by `git-note`".as_bytes())?;
        }
        Ok(())
    }

    pub fn resolve(&self, input: &String) -> anyhow::Result<Paths> {
        let canonical = PathBuf::from(self.root.join(input))
            .canonicalize()
            .context(format!(
                "cannot find specified file `{input}` from {:?}.",
                self.root
            ))?;
        self.validate_path(&canonical, &self.root)?;
        let relative = canonical.strip_prefix(&self.root)?.to_path_buf();
        Ok(Paths::new(self.root.clone(), relative))
    }

    fn validate_path(&self, canonical: &PathBuf, input: &PathBuf) -> anyhow::Result<()> {
        if !canonical.exists() || !canonical.starts_with(input) {
            return Err(anyhow!(
            "specified file `{canonical:?}` looks like not contained in git repository of `{input:?}`",
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
/// - relative : `/bar/baz.txt`
/// - home : `/foo/.git/notes`
/// - note : `/foo/.git/notes/12/34567890`
#[derive(Debug)]
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

    pub fn home(&self) -> anyhow::Result<PathBuf> {
        static NOTE_PATH: &'static str = ".git/notes";

        let path = self.root.join(NOTE_PATH);
        return Ok(path);
    }

    pub fn note(&self, id: &String) -> anyhow::Result<PathBuf> {
        let home = self.home()?;
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
    use crate::testlib::{AnyToString, TestRepo};

    #[test]
    fn path_resolver() -> anyhow::Result<()> {
        let repo = TestRepo::new();
        repo.create_file("foo.txt", Some("hello world"))?;
        let resolver = PathResolver::from_input(repo.path())?;
        println!("root = {:?}", resolver.root);
        Ok(())
    }

    #[test]
    pub fn resolve() -> anyhow::Result<()> {
        let repo = TestRepo::new();
        let path = repo.create_file("foo.txt", Some("hello world"))?;
        let resolver = PathResolver::from_input(repo.path())?;
        let paths = resolver.resolve(&path.str())?;
        assert_eq!(paths.root(), repo.path());
        assert_eq!(paths.canonical(), path);
        assert_eq!(paths.home()?, repo.path().join(".git/notes"));
        assert_eq!(paths.relative(), &PathBuf::from("foo.txt"));
        Ok(())
    }
}
