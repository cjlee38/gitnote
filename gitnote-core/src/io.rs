use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Context;

use crate::libgit::{find_gitnote_path, is_valid_message};
use crate::note::{Message, Note};

pub fn write_note(note: &Note) -> anyhow::Result<()> {
    let note_path = find_note_path(&note.id)?;
    // create a file if it does not exist, and will truncate it if it does.
    let file = File::create(note_path)?;
    serde_json::to_writer(&file, &note)?;
    return Ok(());
}

pub fn read_or_create_note(file_path: &PathBuf) -> anyhow::Result<Note> {
    if let Ok(note) = read_actual_note(file_path) {
        return Ok(note);
    }
    let id = Note::get_id(file_path)?;
    let new = Note::new(&id, file_path);
    write_note(&new)?;
    return Ok(new);
}

pub fn read_actual_note(file_path: &PathBuf) -> anyhow::Result<Note> {
    let id = Note::get_id(file_path)?;
    let note_path = find_note_path(&id)?;

    if let Ok(file) = File::open(&note_path) {
        let reader = BufReader::new(file);
        let messages = serde_json::from_reader(reader)?;
        return Ok(messages);
    }
    let id = Note::get_id(file_path)?;
    return Ok(Note::new(&id, file_path));
}

/// Read note from file and filter out invalid messages
pub fn read_opaque_note(file_path: &PathBuf) -> anyhow::Result<Note> {
    let all_note = read_actual_note(file_path)?;
    let valid_messages: Vec<Message> = all_note.messages.into_iter()
        .filter_map(|message| { is_valid_message(message, file_path) })
        .collect();
    return Ok(Note::from(&all_note.id, &all_note.reference, valid_messages));
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
