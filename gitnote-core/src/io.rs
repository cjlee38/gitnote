use std::cell::{Ref, RefCell};
use std::fs::File;
use std::io::{BufReader, Read};

use itertools::Itertools;

use crate::diff::{DiffModel, GitDiffer};
use crate::libgit::{GitBlob, Libgit};
use crate::note::{Message, Note};
use crate::path::Paths;

pub struct NoteLedger<'p, T>
where
    T: Libgit,
{
    paths: Paths,
    libgit: &'p T,
    note: RefCell<Note>,
}

// TODO : What if opaque line duplicated ?
impl<'p, T> NoteLedger<'p, T>
where
    T: Libgit,
{
    pub fn new(paths: &Paths, libgit: &'p T, note: Note) -> Self {
        Self { paths: paths.clone(), libgit, note: RefCell::new(note) }
    }

    pub fn plain_note(&self) -> Ref<Note> {
        return self.note.borrow();
    }

    pub fn opaque_note(&self) -> Note {
        let note = self.note.borrow();
        let messages = self.opaque_messages();
        return Note::from(&note.id, &note.reference, messages);
    }

    fn plain_messages(&self) -> Ref<Vec<Message>> {
        let note_ref = self.note.borrow();
        return Ref::map(note_ref, |note_ref| &note_ref.messages);
    }

    // todo : review this method.... it is not clear
    /// Read note from file and filter out invalid messages
    fn opaque_messages(&self) -> Vec<Message> {
        let plain = self.plain_messages();
        return plain.iter()
            .filter_map(|m| {
                let old_content = self.libgit.read_content(&self.paths, &m.oid).ok()?;
                let new_blob = self.git_blob().ok()?;

                let mut diff_model = DiffModel::of(m);
                self.libgit.diff(&old_content, &new_blob.content, &mut diff_model);
                if diff_model.valid {
                    Some(m.copied(diff_model.line, new_blob.id.clone()))
                } else {
                    None
                }
            }).collect();
    }

    pub fn git_blob(&self) -> anyhow::Result<GitBlob> {
        return self.libgit.find_volatile_git_blob(&self.paths);
    }

    pub fn opaque_exists(&self, line: usize) -> bool {
        return self.opaque_messages()
            .iter()
            .any(|m| m.line == line);
    }

    pub fn opaque_uuid(&self, line: usize) -> Option<String> {
        let messages = self.opaque_messages();
        return messages.iter()
            .rev()
            .find(|m| m.line == line)
            .map(|m| m.uuid.clone());
    }

    pub fn append(&mut self, line: usize, message: String) -> anyhow::Result<()> {
        let blob = self.git_blob()?;
        let message = Message::new(&blob, line, message)?;
        self.note.borrow_mut().append(message)?;
        return Ok(());
    }

    pub fn delete(&mut self, uuid: String) {
        let mut note_ref = self.note.borrow_mut();
        note_ref.messages.retain(|m| m.uuid != uuid);
    }

    pub fn edit(&self, uuid: String, message: String) {
        self.note.borrow_mut().messages.iter_mut()
            .filter(|m| m.uuid == uuid)
            .for_each(|m| m.update(message.clone()));
    }
}

pub struct NoteRepository<T> where T: Libgit {
    libgit: T
}

impl<T> NoteRepository<T> where T: Libgit {
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

pub fn read_file_content(paths: &Paths) -> anyhow::Result<String> {
    let file = File::open(paths.canonical())?;
    let mut reader = BufReader::new(file);
    // let j = serde_json::from_reader::<BufReader<File>, String>(reader)?;
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    return Ok(content);
}
