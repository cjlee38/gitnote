use std::path::PathBuf;

use anyhow::{anyhow, Context};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::libgit::GitBlob;
use crate::utils::PathBufExt;

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

    pub fn get_id(path: &PathBuf) -> anyhow::Result<String> {
        return Ok(sha256::digest(path.try_to_str()?));
    }

    pub fn append(&mut self, message: Message) -> anyhow::Result<()> {
        self.validate_line_distinct(&message)?;
        self.messages.push(message);
        return Ok(());
    }

    fn validate_line_distinct(&self, message: &Message) -> anyhow::Result<()> {
        if let Some(_) = self.find_message_indexed(message.line) {
            return Err(anyhow!(format!(
                "{} line duplicated. consider to use `edit` instead.", message.line
            )));
        }
        return Ok(());
    }

    pub fn find_message_indexed(&self, line: usize) -> Option<(usize, &Message)> {
        let len = self.messages.len();
        for index in 0..len {
            let message = &self.messages[index];
            if message.line == line {
                return Some((index, &message));
            }
        }
        return None;
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
    created_at: DateTime<Local>,
    #[serde(with = "datetime")]
    pub updated_at: DateTime<Local>,
}

impl Message {
    pub fn new(blob: &GitBlob, line: usize, message: String) -> anyhow::Result<Self> {
        let snippet = blob
            .content
            .get(line)
            .with_context(|| {
                format!("specified line({}) extends limit for file {:?}", line, &blob.file_path)
            })?.to_string();

        Ok(Message {
            uuid: Uuid::new_v4().to_string(),
            oid: blob.id.to_string(),
            line,
            snippet,
            message,
            created_at: Local::now(),
            updated_at: Local::now(),
        })
    }
}

mod datetime {
    use chrono::{DateTime, Local, SecondsFormat};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.to_rfc3339_opts(SecondsFormat::Secs, true);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Local))
            .map_err(serde::de::Error::custom)
    }
}
