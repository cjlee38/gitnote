use std::{fs, io};
use std::path::PathBuf;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::io::{read_or_create_note, write_note};
use crate::libgit::{find_git_blob, find_root_path, GitBlob};

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

        Note { id, content, messages }
    }

    fn append(&mut self, message: Message) {
        self.validate_line_exist(&message);
        // self.validate_line_distinct(&message); // TODO : disable temporarily for development convenience.
        self.messages.push(message);
    }

    fn validate_line_exist(&self, message: &Message) {
        let lines = self.content.len();
        if message.end > lines {
            panic!("given end({}) is too big for content lines {lines}", message.end);
        }
    }

    fn validate_line_distinct(&self, message: &Message) {
        let (start, end) = (message.start, message.end);
        if self.messages.iter().any(|m| m.start == start && m.end == end) {
            panic!("duplicated line");
        }
    }
}

impl Message {
    fn new(start: usize, end: usize, message: String) -> Self {
        if start > end {
            panic!("start({start}) should be lower than end({end})")
        }
        Message { start, end, message }
    }
}

pub fn add_note(file_name: String, line_expr: String, message: String) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;
    let blob = find_git_blob(&file_path)?;

    let (start, end) = parse_line_range(&line_expr).expect("Parsing line range failed");

    let mut note = read_or_create_note(&blob)?;
    println!("===Gitblob is {:?}", blob);

    let message = Message::new(start, end, message);

    note.append(message);
    write_note(&note)?;
    println!("===Note is {:?}", &note);
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
    let path = PathBuf::from(input_path).canonicalize().unwrap();
    let root = find_root_path();

    println!("===resolve_file_path#path : {:?}", &path);
    println!("===resolve_file_path#root : {:?}", &root);
    println!("===resolve_file_path#path.starts_with(root) : {:?}", &path.starts_with(&root));

    if !path.exists() || !path.starts_with(&root) {
        return Err(anyhow!("resolved path is not matched with root path"));
    }
    return Ok(path.strip_prefix(&root)?.to_path_buf());
}

pub fn view_notes(filename: String) -> io::Result<()> {
    // let file = view_matches.get_one::<String>("file").expect("required").clone();
    // let current_dir = std::env::current_dir()?;
    // let notes_dir = current_dir.join(".git_notes");
    // let note_file_path = notes_dir.join(format!("{}_*.json", file));
    //
    // let entries = fs::read_dir(notes_dir)?
    //     .filter_map(|e| e.ok())
    //     .filter(|e| e.path().is_file() && e.path().to_string_lossy().contains(&note_file_path.to_string_lossy()));
    //
    // for entry in entries {
    //     let file = File::open(entry.path())?;
    //     let note: note = serde_json::from_reader(file)?;
    //     println!("{:?}", note);
    // }
    Ok(())
}

pub fn init_notes() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let notes_dir = current_dir.join(".git_notes");
    fs::create_dir_all(&notes_dir)?;
    println!("Initialized the notes directory at {:?}", notes_dir);
    Ok(())
}
