use std::{env, fs};
use std::fs::File;
use std::io::{Write};
use std::path::PathBuf;

use anyhow::{anyhow, Context};
use git2::{Blob, Repository};

#[derive(Debug)]
pub struct GitBlob {
    pub id: String,
    pub file_path: PathBuf,
    pub content: Vec<String>,
}

impl GitBlob {
    pub fn of(blob: Blob, file_path: &PathBuf) -> anyhow::Result<Self> {
        let content_str = std::str::from_utf8(blob.content())
            .map_err(|e| anyhow!("UTF-8 decoding error: {}", e))?;

        return Ok(GitBlob {
            id: blob.id().to_string(),
            file_path: file_path.clone(),
            content: split_lines(content_str),
        });
    }
}

pub fn find_root_path() -> anyhow::Result<PathBuf> {
    Ok(Repository::discover(".")
        .with_context(|| {
            format!(
                "Could not find a Git repository in the current directory : ({:?})",
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
        fs::create_dir(&path).context("Failed to create git-note path; check directory permissions.")?;
        let mut description = File::create(&path.join("description"))?;
        description.write_all("This directory contains notes by `git-note`".as_bytes())?;
    }
    return Ok(path);
}

pub fn find_volatile_git_blob(file_path: &PathBuf) -> anyhow::Result<GitBlob> {
    let repository = Repository::discover(".")?;
    let path = repository.path().parent().unwrap().join(file_path);
    let content = fs::read(&path).context(format!("Failed to read file {:?}", &path))?;
    let oid = repository.blob(&content)?;
    let blob = repository.find_blob(oid)?;
    return GitBlob::of(blob, file_path);
}

pub fn stage_file(file_path: &PathBuf) -> anyhow::Result<()> {
    let repository = Repository::discover(".")?;

    if repository.status_file(file_path)?.is_ignored() {
        return Err(anyhow!("The file {:?} is ignored", file_path));
    }

    let mut index = repository.index()?;
    index.read(true)?;
    index.add_path(file_path)?;
    index.write()?;
    Ok(())
}

fn split_lines(s: &str) -> Vec<String> {
    s.replace("\r\n", "\n")
        .split('\n')
        .map(String::from)
        .collect()
}
