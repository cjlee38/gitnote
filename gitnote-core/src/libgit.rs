use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context};

use crate::diff::{DiffModel, GitDiffer};
use crate::io::read_file_content;
use crate::path::Paths;
use crate::utils::PathBufExt;

#[derive(Debug)]
pub struct GitBlob {
    pub id: String,
    pub file_path: PathBuf,
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
    fn find_volatile_git_blob(&self, paths: &Paths) -> anyhow::Result<GitBlob>;
    fn read_content(&self, paths: &Paths, oid: &String) -> anyhow::Result<String>;
    fn execute_git_command(&self, path: &Path, args: Vec<&str>) -> anyhow::Result<String>;
    fn diff(&self, old: &String, new: &String, diff_model: &mut DiffModel);
}

pub struct ProcessLibgit<T>
where
    T: GitDiffer,
{
    differ: T,
}

impl<T> ProcessLibgit<T>
where
    T: GitDiffer,
{
    pub fn new(differ: T) -> Self {
        Self { differ }
    }
}

impl<T> Libgit for ProcessLibgit<T>
where
    T: GitDiffer,
{
    fn find_volatile_git_blob(&self, paths: &Paths) -> anyhow::Result<GitBlob> {
        let id = self.execute_git_command(
            paths.root(),
            vec!["hash-object", "-w", paths.relative().try_to_str()?],
        )?;
        let content = read_file_content(paths)?;
        Ok(GitBlob {
            id,
            file_path: paths.relative().clone(),
            content: content.replace("\r\n", "\n"),
        })
    }

    fn read_content(&self, paths: &Paths, oid: &String) -> anyhow::Result<String> {
        return Ok(self.execute_git_command(&paths.root(), vec!["cat-file", "-p", oid])?);
    }

    fn execute_git_command(&self, path: &Path, args: Vec<&str>) -> anyhow::Result<String> {
        let output = Command::new("git")
            .args(args.clone())
            .current_dir(path)
            .output()
            .context(format!("!Failed to run `git {:?}`", args))?;
        if !output.status.success() {
            let stderr = std::str::from_utf8(output.stderr.as_slice())
                .map_err(|e| anyhow!("UTF-8 decoding error: {}", e))?;
            return Err(anyhow!("Failed to run `git {args:?}`, error : {stderr}"));
        }
        let result = std::str::from_utf8(output.stdout.as_slice())
            .map_err(|e| anyhow!("UTF-8 decoding error: {}", e))?;
        Ok(result.trim().to_string())
    }

    fn diff(&self, old: &String, new: &String, diff_model: &mut DiffModel) {
        self.differ.diff(old, new, diff_model);
    }
}
