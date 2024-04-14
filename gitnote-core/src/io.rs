use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Context;

use crate::handlers::Note;
use crate::libgit::find_gitnote_path;

pub fn write_note(note: &Note) -> anyhow::Result<()> {
    let note_path = find_note_path(&note.id)?;
    let file = File::create(note_path)?;
    serde_json::to_writer(&file, &note)?;
    return Ok(());
}

pub fn read_or_create_note(file_path: &PathBuf) -> anyhow::Result<Note> {
    if let Ok(note) = read_note(file_path) {
        return Ok(note);
    }
    let id = Note::get_id(file_path)?;
    let new = Note::new(&id, file_path);
    write_note(&new)?;
    return Ok(new);
}

pub fn read_note(file_path: &PathBuf) -> anyhow::Result<Note> {
    let id = Note::get_id(file_path)?;
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
    ensure_dir(&base_path.join(dir))?;
    let note_path = base_path.join(dir).join(file);
    Ok(note_path)
}

fn ensure_dir(dir_path: &PathBuf) -> anyhow::Result<()> {
    if !dir_path.exists() {
        return Ok(std::fs::create_dir(dir_path)
            .context(format!("Failed to create dir for {:?}", dir_path))?)
    }
    return Ok(());
}