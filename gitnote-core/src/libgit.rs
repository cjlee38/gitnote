use fs::File;
use std::fs;
use std::io::{BufWriter, Read, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context};
use flate2::Compression;
use flate2::read::{ZlibDecoder, ZlibEncoder};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::config::CONFIG;
use crate::diff::{Differ, DiffModel};
use crate::path::Paths;
use crate::utils::{create_file_if_not_exists, PathBufExt};

#[derive(Debug)]
pub struct GitBlob {
    pub id: String,
    pub file_path: PathBuf, // relative path
    pub content: String,
}

impl GitBlob {
    pub fn snippet(&self, line: usize) -> Option<String> {
        for (index, snippet) in self.content.lines().enumerate() {
            if index == line {
                return Some(snippet.to_string());
            }
        }
        return None;
    }
}

pub trait Libgit {
    fn make_git_blob(&self, paths: &Paths, persist: bool) -> anyhow::Result<GitBlob>;
    fn read_git_blob(&self, paths: &Paths, oid: &String) -> anyhow::Result<GitBlob>;
    fn diff(&self, old: &String, new: &String, diff_model: &mut DiffModel);

    fn object_path(&self, paths: &Paths, oid: &String) -> PathBuf {
        paths.objects()
            .join(&oid[0..2])
            .join(&oid[2..])
    }
}

pub struct ProcessLibgit<T>
where
    T: Differ,
{
    differ: T,
}

impl<T> ProcessLibgit<T>
where
    T: Differ,
{
    pub fn new(differ: T) -> Self {
        Self { differ }
    }

    fn read_file_content(&self, paths: &Paths) -> anyhow::Result<String> {
        let bytes = fs::read(paths.canonical())?;
        self.decode(&bytes)
            .map_err(|e| anyhow!("Failed to decode file content from `{}` : {}", paths.canonical().display(), e))
    }

    fn decode(&self, bytes: &[u8]) -> anyhow::Result<String> {
        let charset = CONFIG.charset();
        charset.decode(bytes)
            .map_err(|e| anyhow!("Failed to decode file with given charset `{}` : {}", charset, e))
    }

    fn execute_git_command(&self, path: &Path, args: Vec<&str>) -> anyhow::Result<String> {
        let output = Command::new("git")
            .args(args.clone())
            .current_dir(path)
            .output()
            .context(format!("!Failed to run `git {:?}`", args))?;
        if !output.status.success() {
            let stderr = self.decode(&output.stderr)?;
            return Err(anyhow!("Failed to run `git {args:?}`, error : {stderr}"));
        }
        let stdout = self.decode(&output.stdout)?;
        Ok(stdout.trim().to_string())
    }
}

impl<T> Libgit for ProcessLibgit<T>
where
    T: Differ,
{
    fn make_git_blob(&self, paths: &Paths, persist: bool) -> anyhow::Result<GitBlob> {
        let id = self.execute_git_command(
            &paths.root(),
            vec!["hash-object", "-w", paths.relative().try_to_str()?],
        )?;
        let content = self.read_file_content(paths)?;
        Ok(GitBlob {
            id,
            file_path: paths.relative().clone(),
            content: content.replace("\r\n", "\n"),
        })
    }

    fn read_git_blob(&self, paths: &Paths, oid: &String) -> anyhow::Result<GitBlob> {
        let content = self.execute_git_command(&paths.root(), vec!["cat-file", "-p", oid])?;
        Ok(GitBlob {
            id: oid.clone(),
            file_path: paths.relative().clone(),
            content: content.replace("\r\n", "\n"),
        })
    }

    fn diff(&self, old: &String, new: &String, diff_model: &mut DiffModel) {
        self.differ.diff(old, new, diff_model);
    }
}

pub struct ManualLibgit<T>
where
    T: Differ,
{
    differ: T,
}

impl<T> ManualLibgit<T>
where
    T: Differ,
{
    pub fn new(differ: T) -> Self {
        Self { differ }
    }

    fn make_blob_bytes(&self, content: &Vec<u8>) -> Vec<u8> {
        let prefix = "blob ";
        let len = content.len();
        let mut target = format!("{}{}\0", prefix, len).as_bytes().to_vec();
        target.extend(content.iter());
        target
    }

    fn zlib_encode(&self, to_encode: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let mut encoder = ZlibEncoder::new(&to_encode[..], Compression::default());
        let mut encoded = Vec::new();
        encoder.read_to_end(&mut encoded)?;
        Ok(encoded)
    }

    fn oid(&self, content: &Vec<u8>) -> String {
        let mut hasher = sha1_smol::Sha1::new();
        hasher.update(content);
        hasher.digest().to_string()
    }

    fn save_blob(&self, objects_path: PathBuf, oid: &String, encoded: Vec<u8>) -> anyhow::Result<()> {
        let object_dir_path = objects_path.join(&oid[0..2]);
        let object_file_path = &oid[2..];

        create_file_if_not_exists(&object_dir_path, object_file_path, Some(encoded))?;
        Ok(())
    }
}

impl<T> Libgit for ManualLibgit<T>
where
    T: Differ,
{
    fn make_git_blob(&self, paths: &Paths, persist: bool) -> anyhow::Result<GitBlob> {
        let content = fs::read(paths.canonical())
            .context(format!("Failed to read file at path: {:?}", paths.canonical()))?;
        let blob = self.make_blob_bytes(&content);

        let oid = self.oid(&blob);
        if persist {
            let encoded = self.zlib_encode(blob)?;
            self.save_blob(paths.objects(), &oid, encoded)?;
        }

        let blob = GitBlob {
            id: oid,
            file_path: paths.relative(),
            content: String::from_utf8(content)?,
        };
        Ok(blob)
    }

    fn read_git_blob(&self, paths: &Paths, oid: &String) -> anyhow::Result<GitBlob> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"blob \d+\x00").unwrap());

        let blob_path = self.object_path(paths, oid);
        let bytes = fs::read(blob_path)?;
        let mut decoder = ZlibDecoder::new(&bytes[..]);
        let mut blob = String::new();
        decoder.read_to_string(&mut blob)?;
        let content = RE.replacen(&blob, 1, "");

        let git_blob = GitBlob {
            id: oid.clone(),
            file_path: paths.relative().clone(),
            content: content.to_string(),
        };
        Ok(git_blob)
    }

    fn diff(&self, old: &String, new: &String, diff_model: &mut DiffModel) {
        self.differ.diff(old, new, diff_model);
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::SimilarDiffer;
    use crate::libgit::{Libgit, ManualLibgit};
    use crate::path::PathResolver;
    use crate::testlib::TestRepo;

    #[test]
    fn test_make_git_blob() -> anyhow::Result<()> {
        // given
        let repo = TestRepo::new();
        let path = repo.create_file("test.txt", Some("  hello world\nmore lines\n   multiple spaces: and 한글"))?;
        repo.command("git add .")?;
        let oid = "f06840e105b1dd0b30b36bac387239359cd78f99";

        let paths = PathResolver::resolve(repo.path(), "test.txt")?;
        let libgit = ManualLibgit::new(SimilarDiffer);
        let git_blob = libgit.make_git_blob(&paths, false)?;
        assert_eq!(git_blob.id, oid);
        assert_eq!(git_blob.content, "  hello world\nmore lines\n   multiple spaces: and 한글");
        assert_eq!(git_blob.file_path, paths.relative());
        Ok(())
    }

    #[test]
    fn test_read_blob() -> anyhow::Result<()> {
        // given
        let repo = TestRepo::new();
        let path = repo.create_file("test.txt", Some("  hello world\nmore lines\n   multiple spaces: and 한글"))?;
        repo.command("git add .")?;
        let oid = "f06840e105b1dd0b30b36bac387239359cd78f99";

        let paths = PathResolver::resolve(repo.path(), "test.txt")?;
        let libgit = ManualLibgit::new(SimilarDiffer);
        let blob = libgit.read_git_blob(&paths, &oid.to_string())?;
        assert_eq!(blob.content, "  hello world\nmore lines\n   multiple spaces: and 한글");
        Ok(())
    }
}