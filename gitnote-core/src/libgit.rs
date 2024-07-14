use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context};
use similar::{ChangeTag, TextDiff};
use crate::note::Message;

#[derive(Debug)]
pub struct GitBlob {
    pub id: String,
    pub file_path: PathBuf,
    pub content: Vec<String>,
}

pub fn find_root_path() -> anyhow::Result<PathBuf> {
    let path = execute_git_command(vec!["rev-parse", "--show-toplevel"])?;
    path.parse::<PathBuf>().map_err(|e| anyhow!("Failed to parse path: {}", e))
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
    let id = execute_git_command(vec!["hash-object", "-w", file_path.to_str().unwrap()])?;
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(GitBlob { id, file_path: file_path.clone(), content: split_lines(content.as_str()) })
}

pub fn stage_file(file_path: &PathBuf) -> anyhow::Result<()> {
    execute_git_command(vec!["hash-object", "-w", file_path.to_str().unwrap()])?;
    Ok(())
}

pub fn is_valid_message(mut message: Message, file_path: &PathBuf) -> Option<Message> {
    let old_content = execute_git_command(vec!["cat-file", "-p", &message.oid]).ok()?;

    let new_blob = find_volatile_git_blob(file_path).ok()?;
    let binding = new_blob.content.join("\n");
    let new_content = binding.as_str();

    let mut diff_model = DiffModel::of(&message);

    for change in TextDiff::from_lines(old_content.as_str(), new_content).iter_all_changes() {
        let tag = change.tag();
        let old_line = change.old_index().unwrap_or(usize::MAX);
        let new_line = change.new_index().unwrap_or(usize::MAX);

        if old_line == diff_model.line {
            let content = change.value();
            if diff_model.snippet.trim() != content.trim() || (tag == ChangeTag::Delete || tag == ChangeTag::Insert) {
                diff_model.valid = false;
            } else {
                diff_model.line = new_line;
            }
            break;
        }
    }

    if diff_model.valid {
        message.line = diff_model.line;
        message.oid = new_blob.id.to_string();
        return Some(message);
    }
    return None;
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

fn execute_git_command(args: Vec<&str>) -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(args.clone())
        .output()
        .context(format!("Failed to run `git {:?}`", args))?;
    if !output.status.success() {
        return Err(anyhow!("Failed to run `git {:?}`", args));
    }
    let result = std::str::from_utf8(output.stdout.as_slice())
        .map_err(|e| anyhow!("UTF-8 decoding error: {}", e))?;
    Ok(result.trim().to_string())
}
