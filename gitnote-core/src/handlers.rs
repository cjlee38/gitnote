use anyhow::anyhow;
use colored::Colorize;
use unicode_width::UnicodeWidthStr;

use crate::io::{read_actual_note, read_opaque_note, read_or_create_note, write_note};
use crate::libgit::find_volatile_git_blob;
use crate::note::{Message, Note};
use crate::path::Paths;

pub struct NoteHandler;

impl NoteHandler {
    pub fn add_note(&self, paths: &Paths, line: usize, message: String) -> anyhow::Result<()> {
        let mut note = read_or_create_note(paths)?;
        let opaque_note = read_opaque_note(paths)?;
        if opaque_note.find(line).is_some() {
            return Err(anyhow!(
                "comment already exists for line {} in {}. consider to use `edit` instead.",
                line + 1,
                paths
            ));
        }

        let blob = find_volatile_git_blob(paths)?;
        let message = Message::new(&blob, line, message)?;

        note.append(message)?;
        write_note(paths, &note)?;
        return Ok(());
    }

    pub fn read_note(&self, paths: &Paths) -> anyhow::Result<Note> {
        let note = read_opaque_note(paths)?;
        return Ok(note);
    }

    pub fn edit_note(&self, paths: &Paths, line: usize, message: String) -> anyhow::Result<()> {
        let mut actual_note = read_actual_note(paths)?;
        let opaque_note = read_opaque_note(paths)?;

        return if let Some(message_found) = opaque_note.find(line) {
            let uuid = &message_found.uuid;
            for message_to_update in &mut actual_note.messages {
                if message_to_update.uuid == *uuid {
                    message_to_update.message = message.clone();
                    message_to_update.updated_at = chrono::Utc::now();
                }
            }

            write_note(paths, &actual_note)?;
            Ok(())
        } else {
            Err(anyhow!(
                "no comment found for line {} in {}. consider to use `add` instead.",
                line + 1,
                paths
            ))
        };
    }

    pub fn delete_note(&self, paths: &Paths, line: usize) -> anyhow::Result<()> {
        let mut actual_note = read_actual_note(paths)?;
        let opaque_note = read_opaque_note(paths)?;

        return if let Some(message_found) = opaque_note.find(line) {
            let uuid = &message_found.uuid;
            let x: Vec<Message> = actual_note
                .messages
                .into_iter()
                .filter(|m| m.uuid != *uuid)
                .collect();
            actual_note.messages = x;

            write_note(paths, &actual_note)?;
            Ok(())
        } else {
            Err(anyhow!(format!(
                "no comment found for line {} in {:?}",
                line + 1,
                paths
            )))
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::handlers::NoteHandler;
    use crate::io::read_actual_note;
    use crate::note::Note;
    use crate::path::PathResolver;
    use crate::testlib::TestRepo;

    #[test]
    fn test_add_note() -> anyhow::Result<()> {
        // given
        let repo = TestRepo::new();
        let path = repo.create_file("test.txt", Some("foo\nbar\nbaz"))?;
        let path_resolver = PathResolver::from_input(repo.path())?;

        let paths = path_resolver.resolve(&"test.txt".to_string())?;
        let note_handler = NoteHandler;
        note_handler.add_note(&paths, 1, "hello".to_string())?;

        // todo : need assert to test if the note is added correctly
        let note_path = paths.note(&Note::get_id(&paths.relative())?)?;
        assert!(note_path.exists());
        let note = read_actual_note(&paths)?;
        println!("{:?}", note);
        assert_eq!(&note.reference, paths.relative());
        assert_eq!(note.messages.len(), 1usize);
        let message = &note.messages[0];
        assert_eq!(message.message, "hello");
        assert_eq!(message.line, 1);
        assert_eq!(message.snippet, "bar");
        Ok(())
    }

    #[test]
    fn read_note() -> anyhow::Result<()> {
        // given
        let repo = TestRepo::new();
        let path = repo.create_file("test.txt", Some("foo\nbar\nbaz"))?;
        let path_resolver = PathResolver::from_input(repo.path())?;
        let paths = path_resolver.resolve(&"test.txt".to_string())?;
        let note_handler = NoteHandler;

        // when
        note_handler.add_note(&paths, 1, "hello".to_string())?;
        let x = note_handler.read_note(&paths)?;

        // todo : need assert to test if the note is read correctly
        // then
        Ok(())
    }

    #[test]
    fn edit_note() -> anyhow::Result<()> {
        // given
        let repo = TestRepo::new();
        let path = repo.create_file("test.txt", Some("foo\nbar\nbaz"))?;
        let path_resolver = PathResolver::from_input(repo.path())?;
        let paths = path_resolver.resolve(&"test.txt".to_string())?;

        // when
        let note_handler = NoteHandler;
        note_handler.add_note(&paths, 1, "hello".to_string())?;
        let note_path = paths.note(&Note::get_id(&paths.relative())?)?;
        assert!(note_path.exists());
        let note = read_actual_note(&paths)?;
        println!("{:?}", note);
        note_handler.edit_note(&paths, 1, "world".to_string())?;

        // todo : need assert to test if the note is edited correctly
        // then
        Ok(())
    }

    #[test]
    fn delete_note() -> anyhow::Result<()> {
        // given
        let repo = TestRepo::new();
        let path = repo.create_file("test.txt", Some("foo\nbar\nbaz"))?;
        let path_resolver = PathResolver::from_input(repo.path())?;
        let paths = path_resolver.resolve(&"test.txt".to_string())?;

        // when
        let note_handler = NoteHandler;
        note_handler.add_note(&paths, 1, "hello".to_string())?;
        note_handler.delete_note(&paths, 1)?;

        // todo : need assert to test if the note is deleted correctly
        // then
        Ok(())
    }
}
