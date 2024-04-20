use std::path::PathBuf;

use anyhow::{anyhow, Context};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::libgit::GitBlob;

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
        Ok(sha256::digest(path.canonicalize()?.to_str().unwrap()))
    }

    pub fn append(&mut self, message: Message) -> anyhow::Result<()> {
        // self.validate_line_distinct(&message); // TODO : disable temporarily for development convenience.
        self.messages.push(message);
        return Ok(());
    }

    pub fn edit(&mut self, new_message: Message) {
        if let Some((index, _)) = self.find_message_indexed(new_message.start, new_message.end) {
            self.messages.remove(index);
            self.messages.push(new_message);
        }
    }

    pub fn delete(&mut self, start: usize, end: usize) {
        if let Some((index, _)) = self.find_message_indexed(start, end) {
            self.messages.remove(index);
        }
    }

    fn validate_range_distinct(&self, message: &Message) -> anyhow::Result<()> {
        let (start, end) = (message.start, message.end);
        if let None = self.find_message_indexed(start, end) {
            return Err(anyhow!(format!(
                "{start}:{end} line duplicated. consider to use `edit` instead."
            )));
        }
        return Ok(());
    }

    fn find_message_indexed(&self, start: usize, end: usize) -> Option<(usize, &Message)> {
        let len = self.messages.len();
        for index in 0..len {
            let message = &self.messages[index];
            if message.start == start && message.end == end {
                return Some((index, &message));
            }
        }
        return None;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    id: String,
    start: usize,
    end: usize,
    pub snippet: Vec<String>,
    message: String,
    #[serde(with = "datetime")]
    created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(blob: &GitBlob, start: usize, end: usize, message: String) -> anyhow::Result<Self> {
        if start > end {
            return Err(anyhow!("start({start}) should be lower than end({end})"));
        }
        let snippet = blob
            .content
            .get(start - 1..end)
            .with_context(|| {
                format!(
                    "specified end inclusive({}) is too big for file {:?}",
                    end, &blob.file_path
                )
            })?
            .to_vec();

        Ok(Message {
            id: blob.id.to_string(),
            start,
            end,
            snippet,
            message,
            created_at: Utc::now(),
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
