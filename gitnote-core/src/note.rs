use std::cell::{Ref, RefCell};
use std::path::PathBuf;

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::diff::DiffModel;
use crate::libgit::{GitBlob, Libgit};
use crate::path::Paths;
use crate::utils::PathBufExt;

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
        Self {
            paths: paths.clone(),
            libgit,
            note: RefCell::new(note),
        }
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
        return plain
            .iter()
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
            })
            .collect();
    }

    pub fn git_blob(&self) -> anyhow::Result<GitBlob> {
        return self.libgit.find_volatile_git_blob(&self.paths);
    }

    pub fn opaque_exists(&self, line: usize) -> bool {
        return self.opaque_messages().iter().any(|m| m.line == line);
    }

    pub fn opaque_uuid(&self, line: usize) -> Option<String> {
        let messages = self.opaque_messages();
        return messages
            .iter()
            .rev()
            .find(|m| m.line == line)
            .map(|m| m.uuid.clone());
    }

    pub fn append(&self, line: usize, message: String) -> anyhow::Result<()> {
        let blob = self.git_blob()?;
        let message = Message::new(&blob, line, message)?;
        self.note.borrow_mut().append(message)?;
        return Ok(());
    }

    pub fn delete(&self, uuid: String) {
        let mut note_ref = self.note.borrow_mut();
        note_ref.messages.retain(|m| m.uuid != uuid);
    }

    pub fn edit(&self, uuid: String, message: String) {
        self.note
            .borrow_mut()
            .messages
            .iter_mut()
            .filter(|m| m.uuid == uuid)
            .for_each(|m| m.update(message.clone()));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: String,
    pub reference: PathBuf,
    pub messages: Vec<Message>,
}

impl Note {
    pub fn new(id: &String, reference: &PathBuf) -> Self {
        Note::from(id, reference, Vec::new())
    }

    pub fn from(id: &String, reference: &PathBuf, messages: Vec<Message>) -> Self {
        Note {
            id: id.to_owned(),
            reference: reference.to_owned(),
            messages,
        }
    }

    /// Given path should be relative from root of the repository
    pub fn get_id(path: &PathBuf) -> anyhow::Result<String> {
        return Ok(sha256::digest(path.try_to_str()?));
    }

    pub fn messages(&self) -> Vec<&Message> {
        return self.messages.iter().collect();
    }

    pub fn append(&mut self, message: Message) -> anyhow::Result<()> {
        self.messages.push(message);
        return Ok(());
    }

    pub fn find(&self, line: usize) -> Option<&Message> {
        return self.messages.iter().rev().find(|m| m.line == line);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub uuid: String,
    pub oid: String,
    pub line: usize,
    pub snippet: String,
    pub message: String,
    #[serde(with = "datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "datetime")]
    pub updated_at: DateTime<Utc>,
}

impl Message {
    pub fn new(blob: &GitBlob, line: usize, message: String) -> anyhow::Result<Self> {
        let snippet = blob.snippet(line).ok_or(anyhow!(
            "specified line({}) extends limit for file {:?}",
            line,
            &blob.file_path
        ))?;

        Ok(Message {
            uuid: Uuid::new_v4().to_string(),
            oid: blob.id.to_string(),
            line,
            snippet,
            message,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn copied(&self, line: usize, oid: String) -> Self {
        Message {
            uuid: self.uuid.clone(),
            oid,
            line,
            snippet: self.snippet.clone(),
            message: self.message.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }

    pub fn update(&mut self, message: String) {
        self.message = message;
        self.updated_at = Utc::now();
    }
}

mod datetime {
    use chrono::{DateTime, SecondsFormat, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.to_rfc3339_opts(SecondsFormat::Secs, true);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }
}
