use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::from_utf8;

use anyhow::{anyhow, Context};
use git2::{DiffLine, DiffOptions, Oid, Repository};

use crate::libgit::{find_all_git_blobs, find_gitnote_path, GitBlob};
use crate::note::{Message, Note};

pub fn write_note(note: &Note) -> anyhow::Result<()> {
    let note_path = find_note_path(&note.id)?;
    let file = File::create(note_path)?;
    serde_json::to_writer(&file, &note)?;
    return Ok(());
}

pub fn read_or_create_note(file_path: &PathBuf) -> anyhow::Result<Note> {
    if let Ok(note) = read_all_note(file_path) {
        return Ok(note);
    }
    let id = Note::get_id(file_path)?;
    let new = Note::new(&id, file_path);
    write_note(&new)?;
    return Ok(new);
}

pub fn read_all_note(file_path: &PathBuf) -> anyhow::Result<Note> {
    let id = Note::get_id(file_path)?;
    let note_path = find_note_path(&id)?;

    let file = File::open(&note_path)
        .with_context(|| format!("Cannot find the note for path: {:?}", &file_path))?;
    let reader = BufReader::new(file);
    let messages = serde_json::from_reader(reader)?;
    return Ok(messages);
}

pub fn read_valid_note(file_path: &PathBuf) -> anyhow::Result<Note> {
    let git_blobs = find_all_git_blobs(file_path)?;
    if (&git_blobs).is_empty() {
        return Err(anyhow!(format!("No blobs found for {:?}", file_path)));
    }

    let all_note = read_all_note(file_path)?;
    let valid_messages: Vec<Message> = all_note.messages.into_iter()
        .filter_map(|message| {
            is_valid_message(&git_blobs, message, file_path)
        }).collect();
    return Ok(Note::from(&all_note.id, &all_note.reference, valid_messages));
}

fn is_valid_message(git_blobs: &Vec<GitBlob>, mut message: Message, file_path: &PathBuf) -> Option<Message> {
    let repo = Repository::discover(".").ok()?;
    let oid = Oid::from_str(&message.id).ok()?;
    if let Ok(old_blob) = repo.find_blob(oid) {
        // find in index
        if let Some(entry) = repo.index().ok()?.get_path(file_path, 0) {
            let new_blob = repo.find_blob(entry.id).ok()?;
            // early return
            if &old_blob.id() == &new_blob.id() {
                return Some(message);
            }

            // now compare the blobs
            let mut diff_model = DiffModel::of(&message);

            let mut diff_options = DiffOptions::new();
            diff_options.force_text(true);
            diff_options.context_lines(0xFFFFFFF); // max context lines to ensure non-diff lines are included
            repo.diff_blobs(
                Some(&old_blob),
                None,
                Some(&new_blob),
                None,
                Some(&mut diff_options),
                None,
                None,
                None,
                Some(&mut |_, _, l| is_valid_line(&l, &mut diff_model)),
            ).ok()?;

            if diff_model.valid {
                message.line = diff_model.line;
                message.id = git_blobs.last()?.id.clone();
                return Some(message);
            }
        }
        return None;
    }
    return None;
}


#[derive(Debug)]
struct DiffModel {
    line: usize,
    snippet: String,
    valid: bool,
    fixed: bool,
}

impl DiffModel {
    pub fn of(message: &Message) -> Self {
        DiffModel {
            line: message.line,
            snippet: (&message).snippet.to_string(),
            valid: true,
            fixed: false,
        }
    }
}

fn is_valid_line(line: &DiffLine, diff_model: &mut DiffModel) -> bool {
    let old_lineno = line.old_lineno().unwrap_or(0);
    let new_lineno = line.new_lineno().unwrap_or(0);

    if diff_model.fixed {
        return true;
    }

    if old_lineno as usize == diff_model.line {
        line.origin();
        let content = from_utf8(line.content()).unwrap_or("");
        if diff_model.snippet != content.trim() && (line.origin() == '-' || line.origin() == '+') {
            diff_model.valid = false;
        } else {
            diff_model.line = new_lineno as usize;
        }
        diff_model.fixed = true;
    }
    true
}

fn find_note_path(id: &String) -> anyhow::Result<PathBuf> {
    let base_path = find_gitnote_path()?;
    let dir = &id[0..2];
    let file = &id[2..];
    ensure_dir(&base_path.join(dir))?;
    let note_path = base_path.join(dir).join(file);
    Ok(note_path)
}

fn ensure_dir(dir_path: &PathBuf) -> anyhow::Result<()> {
    if !dir_path.exists() {
        return Ok(std::fs::create_dir(dir_path)
            .context(format!("Failed to create directory at path: {:?}", dir_path))?);
    }
    return Ok(());
}
