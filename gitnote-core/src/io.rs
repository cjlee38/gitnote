use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

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

    let file =
        File::open(&note_path).with_context(|| format!("cannot find note : {:?}", &note_path))?;
    let reader = BufReader::new(file);
    let messages = serde_json::from_reader(reader)?;
    return Ok(messages);
}

// TODO : demo-version implementation, so may lead to bad performance.
pub fn read_valid_note(file_path: &PathBuf) -> anyhow::Result<Note> {
    let git_blobs = find_all_git_blobs(file_path)?;
    if (&git_blobs).is_empty() {
        return Err(anyhow!(format!("No blobs found for {:?}", file_path)));
    }

    let all_note = read_all_note(file_path)?;
    let valid_messages: Vec<Message> = all_note.messages.into_iter()
        .filter_map(|message| {
            is_valid_message(&git_blobs, message)
        }).collect();
    return Ok(Note::from(&all_note.id, &all_note.reference, valid_messages));
}

fn is_valid_message(git_blobs: &Vec<GitBlob>, message: Message) -> Option<Message> {
    let repo = Repository::discover(".").ok()?;
    let pos = git_blobs.iter().position(|blob| blob.id == message.id)?;
    let mut diff_model = DiffModel::of(&message);
    let slice = &git_blobs[pos..];
    let mut diff_options = DiffOptions::new();
    diff_options.force_text(true);
    for window in slice.windows(2) {
        let old_blob = repo.find_blob(Oid::from_str(&&window[0].id).ok()?).ok()?;
        let new_blob = repo.find_blob(Oid::from_str(&&window[1].id).ok()?).ok()?;

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
        if !diff_model.valid
            .iter()
            .fold(false, |a, &b| a | b) {
            return None;
        }
        diff_model.valid.clear();
    }
    Some(message)
}


#[derive(Debug)]
struct DiffModel {
    line: usize,
    snippet: String,
    valid: Vec<bool>,
}

impl DiffModel {
    pub fn of(message: &Message) -> Self {
        DiffModel {
            line: message.line,
            snippet: (&message).snippet.to_string(),
            valid: Vec::new(),
        }
    }
}

fn is_valid_line(line: &DiffLine, diff_model: &mut DiffModel) -> bool {
    if line.origin() != ' ' {
        return true;
    }
    let old_line = line.old_lineno()
        .expect("[unexpected error] old_lineno missing");
    let content = std::str::from_utf8(line.content())
        .expect("[unexpected error] content utf-8 missing");
    let new_line = line.new_lineno()
        .expect("[unexpected error] old_lineno missing");

    if diff_model.line == old_line as usize {
        if content.contains(&diff_model.snippet) {
            diff_model.line = new_line as usize;
            diff_model.valid.push(true);
        } else {
            diff_model.valid.push(false);
        }
    }
    return true;
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
            .context(format!("Failed to create dir for {:?}", dir_path))?);
    }
    return Ok(());
}
