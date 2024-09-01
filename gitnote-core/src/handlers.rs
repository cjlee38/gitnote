use anyhow::anyhow;
use colored::Colorize;
use unicode_width::UnicodeWidthStr;

use crate::io::{NoteLedger, NoteRepository};
use crate::libgit::Libgit;
use crate::path::Paths;

pub struct NoteHandler<T>
where
    T: Libgit,
{
    note_repository: NoteRepository<T>,
}

impl<T> NoteHandler<T>
where
    T: Libgit,
{
    pub fn new(note_repository: NoteRepository<T>) -> Self {
        NoteHandler { note_repository }
    }
    pub fn add_note(&self, paths: &Paths, line: usize, message: String) -> anyhow::Result<()> {
        let mut ledger = self.note_repository.read_note(paths)?;
        if ledger.opaque_exists(line) {
            return Err(anyhow!("comment already exists for line {} in {}. consider to use `edit` instead.", line + 1, paths));
        }
        ledger.append(line, message)?;
        self.note_repository.write_note(paths, &ledger.note())?;
        return Ok(());
    }

    pub fn read_note(&self, paths: &Paths) -> anyhow::Result<NoteLedger<T>> {
        let ledger = self.note_repository.read_note(paths)?;
        return Ok(ledger);
    }

    pub fn edit_note(&self, paths: &Paths, line: usize, message: String) -> anyhow::Result<()> {
        let mut ledger = self.note_repository.read_note(paths)?;
        return if let Some(uuid) = ledger.opaque_uuid(line) {
            ledger.edit(uuid, message);
            self.note_repository.write_note(paths, &ledger.note())?;
            Ok(())
        } else {
            Err(anyhow!("no comment found for line {} in {}. consider to use `add` instead.", line + 1, paths))
        };
    }

    pub fn delete_note(&self, paths: &Paths, line: usize) -> anyhow::Result<()> {
        let mut ledger = self.note_repository.read_note(paths)?;

        return if let Some(uuid) = ledger.opaque_uuid(line) {
            ledger.delete(uuid);
            self.note_repository.write_note(paths, &ledger.note())?;
            Ok(())
        } else {
            Err(anyhow!(format!("no comment found for line {} in {:?}",line + 1,paths)))
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::handlers::NoteHandler;
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
        let note = read_note(&paths)?;
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
        let note = read_note(&paths)?;
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
