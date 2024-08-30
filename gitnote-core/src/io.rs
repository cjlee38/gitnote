use std::fs::File;
use std::io::BufReader;

use crate::libgit::is_valid_message;
use crate::note::{Message, Note};
use crate::path::Paths;

pub fn write_note(paths: &Paths, note: &Note) -> anyhow::Result<()> {
    // create a file if it does not exist, and will truncate it if it does.
    let file = File::create(paths.note(&note.id)?)?;
    serde_json::to_writer(&file, &note)?;
    return Ok(());
}

pub fn read_or_create_note(paths: &Paths) -> anyhow::Result<Note> {
    if let Ok(note) = read_actual_note(paths) {
        return Ok(note);
    }
    let id = Note::get_id(paths.relative())?;
    let new = Note::new(&id, paths.relative());
    write_note(paths, &new)?;
    return Ok(new);
}

pub fn read_actual_note(paths: &Paths) -> anyhow::Result<Note> {
    let id = Note::get_id(paths.relative())?;
    let note_path = paths.note(&id)?;

    if let Ok(file) = File::open(&note_path) {
        let reader = BufReader::new(file);
        let messages = serde_json::from_reader(reader)?;
        return Ok(messages);
    }
    let id = Note::get_id(paths.relative())?;
    return Ok(Note::new(&id, paths.relative()));
}

/// Read note from file and filter out invalid messages
pub fn read_opaque_note(paths: &Paths) -> anyhow::Result<Note> {
    let all_note = read_actual_note(paths)?;
    let valid_messages: Vec<Message> = all_note.messages.into_iter()
        .filter_map(|message| { is_valid_message(message, paths) })
        .collect();
    return Ok(Note::from(&all_note.id, &all_note.reference, valid_messages));
}
