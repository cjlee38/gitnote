use std::env;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use colored::Colorize;
use unicode_width::UnicodeWidthStr;

use crate::io::{read_actual_note, read_opaque_note, read_or_create_note, write_note};
use crate::libgit::{find_root_path, find_volatile_git_blob};
use crate::note::Message;
use crate::path::PathResolver;
use crate::stdio::write_out;

pub struct NoteHandler {
    path_resolver: PathResolver,
}

impl NoteHandler {
    pub fn new(current_path: &Path) -> Self {
        NoteHandler {
            path_resolver: PathResolver::from_input(current_path).unwrap()
        }
    }

    pub fn add_note(&self, file_name: String, line: usize, message: String) -> anyhow::Result<()> {
        let line = line - 1; // todo : cli
        let paths = self.path_resolver.resolve(&file_name)?;

        let mut note = read_or_create_note(&paths)?;
        let opaque_note = read_opaque_note(&paths)?;
        if opaque_note.find_message_indexed(line).is_some() {
            return Err(anyhow!(
                "comment already exists for line {} in {}. consider to use `edit` instead.",
                line + 1,
                &paths
            ));
        }

        let blob = find_volatile_git_blob(&paths)?;
        let message = Message::new(&blob, line, message)?;

        note.append(message)?;
        write_note(&paths, &note)?;
        write_out(&format!( // todo : cli
            "Successfully added comment for {} in range {}",
                            &paths,
                            line + 1
        ));
        return Ok(());
    }

    pub fn read_note(&self, file_name: String, formatted: bool) -> anyhow::Result<()> {
        let paths = self.path_resolver.resolve(&file_name)?;
        let blob = find_volatile_git_blob(&paths)?;

        let note = read_opaque_note(&paths)?;
        let content = &blob.content;

        if formatted {
            let note_str = serde_json::to_string_pretty(&note)?;
            write_out(&note_str);
            return Ok(());
        }
        let messages = note.messages;
        content.lines().enumerate().for_each(|(line, line_content)| {
            let message = messages.iter().rev().find(|m| m.line == line);
            if let Some(found) = message {
                let message_lines = found
                    .message
                    .split("\n")
                    .map(String::from)
                    .collect::<Vec<String>>();
                let padding = found.snippet.width();
                for (i, line) in message_lines.iter().enumerate() {
                    if i == 0 {
                        write_out(&format!(
                            "{} {} {} ",
                            (found.line + 1).to_string().yellow(),
                            found.snippet,
                            line.red()
                        ));
                    } else {
                        write_out(&format!(
                            "{:width$} {}",
                            "",
                            line.red(),
                            width = padding + 2
                        ));
                    }
                }
            } else {
                write_out(&format!(
                    "{} {}",
                    (line + 1).to_string().yellow(),
                    line_content
                ));
            }
        });

        Ok(())
    }

    pub fn edit_note(&self, file_name: String, line: usize, message: String) -> anyhow::Result<()> {
        let line = line - 1;
        let paths = self.path_resolver.resolve(&file_name)?;

        let mut actual_note = read_actual_note(&paths)?;
        let opaque_note = read_opaque_note(&paths)?;

        return if let Some((_, message_found)) = opaque_note.find_message_indexed(line) {
            let uuid = &message_found.uuid;
            for message_to_update in &mut actual_note.messages {
                if message_to_update.uuid == *uuid {
                    message_to_update.message = message.clone();
                    message_to_update.updated_at = chrono::Utc::now();
                }
            }

            write_note(&paths, &actual_note)?;
            Ok(())
        } else {
            Err(anyhow!(format!(
            "no comment found for line {} in {}. consider to use `add` instead.",
            line + 1,
            &paths
        )))
        };
    }

    pub fn delete_note(&self, file_name: String, line: usize) -> anyhow::Result<()> {
        let line = line - 1;
        let paths = self.path_resolver.resolve(&file_name)?;

        let mut actual_note = read_actual_note(&paths)?;
        let opaque_note = read_opaque_note(&paths)?;

        return if let Some((_, message_found)) = opaque_note.find_message_indexed(line) {
            let uuid = &message_found.uuid;
            let x: Vec<Message> = actual_note
                .messages
                .into_iter()
                .filter(|m| m.uuid != *uuid)
                .collect();
            actual_note.messages = x;

            write_note(&paths, &actual_note)?;
            Ok(())
        } else {
            Err(anyhow!(format!(
            "no comment found for line {} in {:?}",
            line + 1,
            &paths
        )))
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::handlers::NoteHandler;
    use crate::note::Note;
    use crate::testlib::TestRepo;

    #[test]
    fn test_add_note() -> anyhow::Result<()> {
        // given
        let repo = TestRepo::new();
        let path = repo.create_file("test.txt", Some("foo\nbar\nbaz")).unwrap();
        let note_handler = NoteHandler::new(repo.path());
        note_handler.add_note("test.txt".to_string(), 1, "hello".to_string())?;

        // todo : need assert to test if the note is added correctly
        Ok(())
    }

    #[test]
    fn read_note() -> anyhow::Result<()> {
        // given
        // let repo = TestRepo::new();
        // let path = repo.create_file("test.txt", Some("foo\nbar\nbaz")).unwrap();
        // let note_handler = NoteHandler::new(repo.path());
        //
        // let home = note_handler.path_resolver.home()?;
        // // Note::get_id()
        // // home.
        // note_handler.add_note("test.txt".to_string(), 1, "hello".to_string())?;
        // note_handler.read_note("test.txt".to_string(), false)?;
        Ok(())
    }
}
