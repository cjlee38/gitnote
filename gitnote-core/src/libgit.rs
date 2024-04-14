use std::env;
use std::path::PathBuf;

use anyhow::{anyhow, Context};
use git2::{Error, Repository, StatusOptions, StatusShow};

#[derive(Debug)]
pub struct GitBlob {
    pub id: String,
    pub content: Vec<String>,
}

pub fn find_root_path() -> anyhow::Result<PathBuf> {
    Ok(Repository::discover(".")
        .with_context(|| {
            format!(
                "git repository not found for current directry({:?})",
                env::current_dir().unwrap()
            )
        })?
        .workdir()
        .context("git repository working directory not found")?
        .to_path_buf())
}

pub fn find_gitnote_path() -> anyhow::Result<PathBuf> {
    let path = find_root_path()?.join(PathBuf::from(".git/.git-notes"));
    if let Ok(false) = path.try_exists().context(
        "failed to find git-note path. consider to check the permission of your directory.",
    ) {
        std::fs::create_dir(&path).context(
            "failed to create git-note path. consider to check the permission of your directory.",
        )?;
    }
    return Ok(path);
}

pub fn find_git_blob(file_path: &PathBuf) -> anyhow::Result<GitBlob> {
    let repository = Repository::discover(".")?;

    let head = repository.head()?.resolve()?.peel_to_commit()?;
    let object = head.tree()?.get_path(file_path)?.to_object(&repository)?;

    if let Some(blob) = object.as_blob() {
        let id = blob.id().to_string();
        let content_bytes = blob.content();
        let content_str =
            std::str::from_utf8(content_bytes) // TODO : What if content is not utf8 ?
                .map_err(|e| Error::from_str(&format!("UTF-8 decoding error: {}", e)))?;
        let content = split_lines(content_str);

        Ok(GitBlob { id, content })
    } else {
        Err(anyhow!("Not a blob"))
    }
}

pub fn is_file_staged(file_path: &PathBuf) -> anyhow::Result<bool> {
    let file_path_str = file_path.to_str().unwrap();
    let repository = Repository::discover(".")?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true)
        .include_unmodified(true)
        .show(StatusShow::IndexAndWorkdir);

    let statuses = repository.statuses(Some(&mut opts))?;
    print!("statussesssss {:?}", &statuses.len());
    let entry = statuses.iter()
        .find(|entry| {
            println!("=== iter {:?}, status {:?}", &entry.path(), &entry.status());
            entry.path().unwrap_or_default() == file_path_str
        });

    return match entry {
        Some(entry) => {
            println!("=== entry status = {:?}", &entry.status());
            Ok(entry.status().bits() <= (1 << 4))
        },
        None => Ok(false),
    };
}

fn split_lines(s: &str) -> Vec<String> {
    s.replace("\r\n", "\n")
        .split('\n')
        .map(String::from)
        .collect()
}
