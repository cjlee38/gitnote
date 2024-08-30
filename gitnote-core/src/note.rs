use std::path::PathBuf;

use anyhow::{anyhow, Context};
use chrono::{DateTime, Utc};
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

    /// Given path should be relative from root of the repository
    pub fn get_id(path: &PathBuf) -> anyhow::Result<String> {
        return Ok(sha256::digest(path.try_to_str()?));
    }

    pub fn append(&mut self, message: Message) -> anyhow::Result<()> {
        self.messages.push(message);
        return Ok(());
    }

    pub fn find_message_indexed(&self, line: usize) -> Option<(usize, &Message)> {
        return self.messages.iter()
            .enumerate()
            .find(|(i, m)| m.line == line);
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
        let snippet = blob.snippet(line)
            .ok_or(anyhow!("specified line({}) extends limit for file {:?}", line, &blob.file_path))?;

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
