use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{anyhow, Context};
use git2::{Blob, Repository, StatusOptions, StatusShow};

#[derive(Debug)]
pub struct GitBlob {
    pub id: String,
    pub file_path: PathBuf,
    pub content: Vec<String>,
}

pub fn find_root_path() -> anyhow::Result<PathBuf> {
    Ok(Repository::discover(".")
        .with_context(|| {
            format!(
                "git repository not found for current directory({:?})",
                env::current_dir().unwrap()
            )
        })?
        .workdir()
        .context("git repository working directory not found")?
        .to_path_buf())
}

pub fn find_gitnote_path() -> anyhow::Result<PathBuf> {
    let path = find_root_path()?.join(PathBuf::from(".git/notes"));
    if !path.try_exists().context("Failed to access the git-note path; check directory permissions.")? {
        std::fs::create_dir(&path).context("Failed to create git-note path; check directory permissions.")?;
        let mut description = File::create(&path.join("description"))?;
        description.write_all("This directory contains notes by `git-note`".as_bytes())?;
    }
    return Ok(path);
}

pub fn find_git_blob(file_path: &PathBuf) -> anyhow::Result<GitBlob> {
    let repository = Repository::discover(".")?;

    let blob_to_git_blob = |blob: Blob, file_path: &PathBuf| -> anyhow::Result<GitBlob> {
        let content_str = std::str::from_utf8(blob.content())
            .map_err(|e| anyhow!("UTF-8 decoding error: {}", e))?;
        Ok(GitBlob {
            id: blob.id().to_string(),
            file_path: file_path.clone(),
            content: split_lines(content_str),
        })
    };

    // Check index first
    let index = repository.index()?;
    if let Some(entry) = index.get_path(file_path, 0) {
        let blob = repository.find_blob(entry.id)?;
        return blob_to_git_blob(blob, file_path);
    }

    // If not found in index, check HEAD commit
    let head = repository.head()?.resolve()?.peel_to_commit()?;
    let object = head.tree()?.get_path(file_path)?.to_object(&repository)?;
    if let Some(blob) = object.as_blob() {
        return blob_to_git_blob(blob.clone(), file_path);
    }

    Err(anyhow!("File not found as a blob in the index or in the repository HEAD"))
}

pub fn is_file_staged(file_path: &PathBuf) -> anyhow::Result<bool> {
    let file_path_str = file_path.to_str().unwrap();
    let repository = Repository::discover(".")?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true)
        .include_unmodified(true)
        .show(StatusShow::IndexAndWorkdir);

    let statuses = repository.statuses(Some(&mut opts))?;
    let entry = statuses.iter()
        .find(|entry| {
            entry.path().unwrap_or_default() == file_path_str
        });

    return match entry {
        Some(entry) => Ok(entry.status().bits() <= (1 << 4)), // CURRENT + INDEXED_XXX
        None => Ok(false),
    };
}

fn split_lines(s: &str) -> Vec<String> {
    s.replace("\r\n", "\n")
        .split('\n')
        .map(String::from)
        .collect()
}
