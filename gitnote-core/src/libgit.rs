use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{anyhow, Context};
use git2::{Blob, Repository};
use linked_hash_set::LinkedHashSet;

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
        std::fs::create_dir(&path).context("Failed to create git-note path; check directory permissions.")?;
        let mut description = File::create(&path.join("description"))?;
        description.write_all("This directory contains notes by `git-note`".as_bytes())?;
    }
    return Ok(path);
}

pub fn find_git_blob(file_path: &PathBuf) -> anyhow::Result<GitBlob> {
    let repository = Repository::discover(".")?;

    // Check index first
    let index = repository.index()?;
    if let Some(entry) = index.get_path(file_path, 0) {
        let blob = repository.find_blob(entry.id)?;
        return GitBlob::of(blob, file_path);
    }

    // If not found in index, check HEAD commit
    let head = repository.head()?.resolve()?.peel_to_commit()?;
    let object = head.tree()?.get_path(file_path)?.to_object(&repository)?;
    if let Some(blob) = object.as_blob() {
        return GitBlob::of(blob.clone(), file_path)
    }

    return Err(anyhow!("The file was not found in the repository's index or in the latest commit"));
}

// TODO : It only returns the blobs which contained in commits.
//   There could be several ways to handle this problem...
//   1) Notify user that the file is in dangling state.
//   2) Find the blob any how -> Might to lead loss of data
pub fn find_all_git_blobs(file_path:&PathBuf) -> anyhow::Result<Vec<GitBlob>> {
    let repo = Repository::discover(".")?;
    let mut oids = LinkedHashSet::new();
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for commit_id in revwalk {
        if let Err(_e) = commit_id {
            return Err(anyhow!(format!("Could not find the commit ID while walking through the repository history for file: {:?}", file_path)));
        }
        let commit = repo.find_commit(commit_id.unwrap())?;
        let tree = commit.tree()?;
        if let Ok(entry) = tree.get_path(file_path) {
            if entry.kind() == Some(git2::ObjectType::Blob) {
                oids.insert(entry.id());
            }
        }
    }
    let mut blobs: Vec<GitBlob> = Vec::new();
    for oid in oids {
        if let Ok(blob) = repo.find_blob(oid) {
            blobs.push(GitBlob::of(blob, file_path)?);
        }
    }
    blobs.reverse();
    return Ok(blobs);
}

pub fn stage_file(file_path: &PathBuf) -> anyhow::Result<()> {
    let repository = Repository::discover(".")?;

    let mut index = repository.index()?;
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
