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
        self.note_repository.write_note(paths, &ledger.plain_note())?;
        return Ok(());
    }

    pub fn read_note(&self, paths: &Paths) -> anyhow::Result<NoteLedger<T>> {
        let ledger = self.note_repository.read_note(paths)?;
        return Ok(ledger);
    }

    pub fn edit_note(&self, paths: &Paths, line: usize, message: String) -> anyhow::Result<()> {
        let mut ledger = self.note_repository.read_note(paths)?;
        return if let Some(uuid) = ledger.opaque_uuid(line) {
            println!("editing message found uuid {}", uuid);
            ledger.edit(uuid, message);
            self.note_repository.write_note(paths, &ledger.plain_note())?;
            Ok(())
        } else {
            Err(anyhow!("no comment found for line {} in {}. consider to use `add` instead.", line + 1, paths))
        };
    }

    pub fn delete_note(&self, paths: &Paths, line: usize) -> anyhow::Result<()> {
        let mut ledger = self.note_repository.read_note(paths)?;

        return if let Some(uuid) = ledger.opaque_uuid(line) {
            ledger.delete(uuid);
            self.note_repository.write_note(paths, &ledger.plain_note())?;
            Ok(())
        } else {
            Err(anyhow!(format!("no comment found for line {} in {:?}",line + 1,paths)))
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::SimilarGitDiffer;
    use crate::handlers::NoteHandler;
    use crate::io::NoteRepository;
    use crate::libgit::{Libgit, ProcessLibgit};
    use crate::note::Note;
    use crate::path::{PathResolver, Paths};
    use crate::testlib::TestRepo;

    struct Sut {
        repo: TestRepo,
        paths: Paths,
        note_handler: NoteHandler<ProcessLibgit<SimilarGitDiffer>>,
    }

    impl Sut {
        fn setup(content: &str) -> anyhow::Result<Self> {
            let repo = TestRepo::new();
            let path = repo.create_file("test.txt", Some(content))?;
            let libgit = ProcessLibgit::new(SimilarGitDiffer);
            let path_resolver = PathResolver::from_input(repo.path(), &libgit)?;

            let paths = path_resolver.resolve(&"test.txt".to_string())?;
            let repository = NoteRepository::new(libgit);
            let note_handler = NoteHandler::new(repository);
            Ok(Sut { repo, paths, note_handler })
        }
    }

    #[test]
    fn test_add_note() -> anyhow::Result<()> {
        // given
        let sut = Sut::setup("foo\nbar\nbaz")?;

        // when
        sut.note_handler.add_note(&sut.paths, 1, "hello".to_string())?;

        // then
        let note_path = sut.paths.note(&Note::get_id(&sut.paths.relative())?)?;
        assert!(note_path.exists());

        let note: Note = sut.repo.read_note(&note_path)?;
        assert_eq!(&note.reference, sut.paths.relative());
        assert_eq!(note.messages.len(), 1);
        let message = &note.messages[0];
        assert_eq!(message.message, "hello");
        assert_eq!(message.line, 1);
        assert_eq!(message.snippet, "bar");
        Ok(())
    }

    #[test]
    fn test_read_note() -> anyhow::Result<()> {
        // given
        let sut = Sut::setup("foo\nbar\nbaz")?;

        // when
        sut.note_handler.add_note(&sut.paths, 1, "hello".to_string())?;
        let ledger = sut.note_handler.read_note(&sut.paths)?;

        // then
        let note = ledger.plain_note();
        assert_eq!(&note.reference, sut.paths.relative());
        assert_eq!(note.messages.len(), 1);
        let message = &note.messages[0];
        assert_eq!(message.message, "hello");
        assert_eq!(message.line, 1);
        assert_eq!(message.snippet, "bar");
        Ok(())
    }

    #[test]
    fn edit_note() -> anyhow::Result<()> {
        // given
        let sut = Sut::setup("foo\nbar\nbaz")?;

        // when
        sut.note_handler.add_note(&sut.paths, 1, "hello".to_string())?;
        sut.note_handler.edit_note(&sut.paths, 1, "world".to_string())?;

        // then
        let note_path = sut.paths.note(&Note::get_id(&sut.paths.relative())?)?;
        assert!(note_path.exists());

        let note: Note = sut.repo.read_note(&note_path)?;
        assert_eq!(&note.reference, sut.paths.relative());
        assert_eq!(note.messages.len(), 1);
        let message = &note.messages[0];
        assert_eq!(message.message, "world");
        assert_eq!(message.line, 1);
        assert_eq!(message.snippet, "bar");

        Ok(())
    }

    #[test]
    fn delete_note() -> anyhow::Result<()> {
        // given
        let sut = Sut::setup("foo\nbar\nbaz")?;

        // when
        sut.note_handler.add_note(&sut.paths, 1, "hello".to_string())?;
        sut.note_handler.delete_note(&sut.paths, 1)?;

        // then
        let note_path = sut.paths.note(&Note::get_id(&sut.paths.relative())?)?;
        assert!(note_path.exists());

        let note: Note = sut.repo.read_note(&note_path)?;
        assert_eq!(&note.reference, sut.paths.relative());
        assert_eq!(note.messages.len(), 0);

        Ok(())
    }
}
