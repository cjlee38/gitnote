use std::fmt::format;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Error};
use serde::{Deserialize, Serialize};

use crate::io::{read_note, read_or_create_note, write_note};
use crate::libgit::{find_git_blob, find_root_path, is_file_staged, GitBlob};

#[derive(Debug)]
pub struct Note {
    pub id: String,
    pub content: Vec<String>,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    start: usize,
    end: usize,
    message: String,
}

impl Note {
    pub(crate) fn new(blob: &GitBlob) -> Self {
        Note::from(blob, Vec::new())
    }

    pub(crate) fn from(blob: &GitBlob, messages: Vec<Message>) -> Self {
        let id = blob.id.to_owned();
        let content = blob.content.to_vec();

        Note {
            id,
            content,
            messages,
        }
    }

    fn append(&mut self, message: Message) {
        self.validate_range_exists(&message);
        // self.validate_line_distinct(&message); // TODO : disable temporarily for development convenience.
        self.messages.push(message);
    }

    fn edit(&mut self, new_message: Message) {
        if let Some((index, _)) = self.find_message_indexed(new_message.start, new_message.end) {
            self.messages.remove(index);
            self.messages.push(new_message);
        }
    }

    fn delete(&mut self, start: usize, end: usize) {
        if let Some((index, _)) = self.find_message_indexed(start, end) {
            self.messages.remove(index);
        }
    }

    fn validate_range_exists(&self, message: &Message) -> anyhow::Result<()> {
        let lines = self.content.len();
        if message.end > lines {
            return Err(anyhow!(format!(
                "given end({}) is too big for content lines {lines}",
                message.end
            )));
        }
        return Ok(());
    }

    fn validate_range_distinct(&self, message: &Message) -> anyhow::Result<()> {
        let (start, end) = (message.start, message.end);
        if let None = self.find_message_indexed(start, end) {
            return Err(anyhow!(format!(
                "{start}:{end} line duplicated. consider to use `edit` instead."
            )));
        }
        return Ok(());
    }

    fn find_message_indexed(&self, start: usize, end: usize) -> Option<(usize, &Message)> {
        let len = self.messages.len();
        for index in 0..len {
            let message = &self.messages[index];
            if message.start == start && message.end == end {
                return Some((index, &message));
            }
        }
        return None;
    }
}

impl Message {
    fn new(start: usize, end: usize, message: String) -> anyhow::Result<Self> {
        if start > end {
            return Err(anyhow!("start({start}) should be lower than end({end})"));
        }
        Ok(Message {
            start,
            end,
            message,
        })
    }
}

pub fn add_note(file_name: String, line_expr: String, message: String) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;
    if !is_file_staged(&file_path)? {
        return Err(anyhow!(format!(
            "file \"{}\" is not up-to-date. stage the file using `git add {}` before add comment",
            &file_name, &file_name
        )));
    }
    let blob = find_git_blob(&file_path)?;
    let (start, end) = parse_line_range(&line_expr)?;

    let mut note = read_or_create_note(&blob)?;
    let message = Message::new(start, end, message)?;
    note.append(message);
    write_note(&note)?;

    return Ok(());
}

fn parse_line_range(line_expr: &str) -> anyhow::Result<(usize, usize)> {
    let parts: Vec<&str> = line_expr.split(':').collect();
    match parts.len() {
        1 => {
            let line = parts[0].parse::<usize>()?;
            Ok((line, line))
        }
        2 => {
            let start = parts[0].parse::<usize>()?;
            let end = parts[1].parse::<usize>()?;
            Ok((start, end))
        }
        _ => Err(anyhow!("invalid line range format : {line_expr}")),
    }
}

fn resolve_path(input_path: &String) -> anyhow::Result<PathBuf> {
    let abs_path = PathBuf::from(input_path)
        .canonicalize()
        .with_context(|| format!("cannot find to specified file [{input_path}]."))?;
    let root_path = find_root_path()?;

    if !abs_path.exists() || !abs_path.starts_with(&root_path) {
        return Err(anyhow!(format!(
            "specified file {:?} looks like not contained in git repository of {:?}",
            abs_path, root_path
        )));
    }
    return Ok(abs_path.strip_prefix(&root_path)?.to_path_buf());
}

pub fn read_notes(file_name: String) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;
    let blob = find_git_blob(&file_path)?;
    let note = read_note(&blob)?;

    println!("===view note : {:?}", note);
    Ok(())
}

pub fn edit_note(file_name: String, line_expr: String, message: String) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;
    if !is_file_staged(&file_path)? {
        return Err(anyhow!(format!(
            "file \"{}\" is not up-to-date. stage the file using `git add {}` before add comment",
            &file_name, &file_name
        )));
    }
    let blob = find_git_blob(&file_path)?;
    let (start, end) = parse_line_range(&line_expr)?;

    let mut note = read_note(&blob)?;
    let message = Message::new(start, end, message)?;
    note.edit(message);

    write_note(&note)?;
    return Ok(());
}

pub fn delete_note(file_name: String, line_expr: String) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;
    let blob = find_git_blob(&file_path)?;
    let (start, end) = parse_line_range(&line_expr)?;

    let mut note = read_note(&blob)?;
    note.delete(start, end);
    write_note(&note)?;
    return Ok(());
}
