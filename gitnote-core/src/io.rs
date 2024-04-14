use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::{anyhow, Context};

use crate::handlers::{Message, Note};
use crate::libgit::{find_gitnote_path, GitBlob};

pub fn write_note(note: &Note) -> anyhow::Result<()> {
    let note_path = find_note_path(&note.id)?;
    let file = File::create(note_path)?;
    serde_json::to_writer(&file, &note.messages)?;
    return Ok(());
}

pub fn read_or_create_note(blob: &GitBlob) -> anyhow::Result<Note> {
    if let Ok(note) = read_note(blob) {
        return Ok(note);
    }
    let new = Note::new(blob);
    write_note(&new)?;
    return Ok(new);
}

pub fn read_note(blob: &GitBlob) -> anyhow::Result<Note> {
    let messages = read_messages(&blob.id);
    match messages {
        Ok(m) => Ok(Note::from(blob, m)),
        Err(e) => Err(anyhow!("cannot read note. {:?}", e)),
    }
}

fn read_messages(id: &String) -> anyhow::Result<Vec<Message>>{
    let note_path = find_note_path(&id)?;

    let file = File::open(&note_path).with_context(|| format!("cannot find note : {:?}", &note_path))?;
    let reader = BufReader::new(file);
    let messages = serde_json::from_reader(reader)?;
    return Ok(messages);
}

fn find_note_path(id: &String) -> anyhow::Result<PathBuf> {
    let base_path = find_gitnote_path()?;
    let dir = &id[0..2];
    let file = &id[2..];
    ensure_dir(&base_path.join(dir));
    let note_path = base_path.join(dir).join(file);
    Ok(note_path)
}

fn ensure_dir(dir_path: &PathBuf) {
    if !dir_path.exists() {
        std::fs::create_dir(dir_path)
            .expect(&format!("Failed to create dir for {:?}", dir_path));
    }
}