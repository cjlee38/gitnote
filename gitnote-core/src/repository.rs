use std::fs::File;
use std::io::BufReader;

use crate::diff::GitDiffer;
use crate::libgit::Libgit;
use crate::note::{Note, NoteLedger};
use crate::path::Paths;

pub struct NoteRepository<T>
where
    T: Libgit,
{
    libgit: T,
}

impl<T> NoteRepository<T>
where
    T: Libgit,
{
    pub fn new(libgit: T) -> Self {
        Self { libgit }
    }

    pub fn write_note(&self, paths: &Paths, note: &Note) -> anyhow::Result<()> {
        // create a file if it does not exist, and will truncate it if it does.
        let file = File::create(paths.note(&note.id)?)?;
        serde_json::to_writer(&file, note)?;
        return Ok(());
    }

    pub fn read_note(&self, paths: &Paths) -> anyhow::Result<NoteLedger<T>> {
        let ledger = self.do_read_note(paths)?;
        return Ok(ledger);
    }

    fn do_read_note(&self, paths: &Paths) -> anyhow::Result<NoteLedger<T>> {
        let id = Note::get_id(paths.relative())?;
        let note_path = paths.note(&id)?;

        let note = if note_path.exists() {
            let file = File::open(&note_path)?;
            serde_json::from_reader(BufReader::new(file))?
        } else {
            let note = Note::new(&id, paths.relative());
            self.write_note(paths, &note)?;
            note
        };
        return Ok(NoteLedger::new(paths, &self.libgit, note));
    }
}
