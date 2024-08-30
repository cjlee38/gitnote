use std::{env, fs};
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Index;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context};
use similar::{ChangeTag, TextDiff};

use crate::note::Message;
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

#[deprecated]
pub fn find_root_path(path: &Path) -> anyhow::Result<PathBuf> {
    execute_git_command(path, vec!["rev-parse", "--show-toplevel"])?
        .parse::<PathBuf>()
        .map_err(|e| anyhow!("Failed to parse path: {}", e))
}

#[deprecated]
pub fn find_gitnote_path() -> anyhow::Result<PathBuf> {
    let path = find_root_path(&env::current_dir()?)?.join(".git/notes");
    if !path
        .try_exists()
        .context("Failed to access the git-note path; check directory permissions.")?
    {
        initialize_note_path(&path)?;
    }
    return Ok(path);
}

#[deprecated]
fn initialize_note_path(path: &PathBuf) -> anyhow::Result<()> {
    fs::create_dir(&path)
        .context("Failed to create git-note path; check directory permissions.")?;
    let mut description = File::create(&path.join("description"))?;
    description.write_all("This directory contains notes by `git-note`".as_bytes())?;
    Ok(())
}

pub fn find_volatile_git_blob(paths: &Paths) -> anyhow::Result<GitBlob> {
    let id = execute_git_command(
        paths.root(),
        vec!["hash-object", "-w", paths.relative().try_to_str()?]
    )?;
    let mut file = File::open(paths.canonical())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(GitBlob {
        id,
        file_path: paths.relative().clone(),
        content: content.replace("\r\n", "\n"),
    })
}

pub fn is_valid_message(mut message: Message, paths: &Paths) -> Option<Message> {
    let old_content = execute_git_command(&paths.root(), vec!["cat-file", "-p", &message.oid]).ok()?;

    let new_blob = find_volatile_git_blob(paths).ok()?;
    let mut diff_model = DiffModel::of(&message);
    diff(old_content, new_blob.content, &mut diff_model);

    if diff_model.valid {
        message.line = diff_model.line;
        message.oid = new_blob.id;
        return Some(message);
    }
    return None;
}

fn diff(old: String, new: String, diff_model: &mut DiffModel) {
    for change in TextDiff::from_lines(&old, &new).iter_all_changes() {
        let tag = change.tag();
        let old_line = change.old_index().unwrap_or(usize::MAX);
        let new_line = change.new_index().unwrap_or(usize::MAX);

        if old_line == diff_model.line {
            let content = change.value();
            if diff_model.snippet.trim() != content.trim()
                || (tag == ChangeTag::Delete || tag == ChangeTag::Insert)
            {
                diff_model.valid = false;
            } else {
                diff_model.line = new_line;
            }
            break;
        }
    }
}

#[derive(Debug)]
struct DiffModel {
    line: usize,
    snippet: String,
    valid: bool,
}

impl DiffModel {
    pub fn of(message: &Message) -> Self {
        DiffModel {
            line: message.line,
            snippet: (&message).snippet.to_string(),
            valid: true,
        }
    }
}

fn split_lines(s: &str) -> Vec<String> {
    s.replace("\r\n", "\n")
        .split('\n')
        .map(String::from)
        .collect()
}

pub fn execute_git_command(path: &Path, args: Vec<&str>) -> anyhow::Result<String> {
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
