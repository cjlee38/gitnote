use std::fs::{create_dir_all, File};
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Context;

use crate::handlers::Note;
use crate::libgit::find_gitnote_path;

pub fn write_note(note: &Note) -> anyhow::Result<()> {
    let note_path = find_note_path(&note.id)?;
    let file = File::create(note_path)?;
    serde_json::to_writer(&file, note)?;
    return Ok(());
}

pub fn read_or_create_note(id: &String) -> anyhow::Result<Note> {
    if let Ok(note) = read_note(id) {
        return Ok(note)
    }
    let new = Note::new(id);
    write_note(&new)?;
    return Ok(new);
}

pub fn read_note(id: &String) -> anyhow::Result<Note>{
    let note_path = find_note_path(&id)?;

    let file = File::open(&note_path).with_context(|| format!("cannot find note : {:?}", &note_path))?;
    let reader = BufReader::new(file);
    let note = serde_json::from_reader(reader)?;
    return Ok(note);
}

fn find_note_path(id: &String) -> anyhow::Result<PathBuf> {
    let base_path = find_gitnote_path();
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