use anyhow::anyhow;
use colored::Colorize;
use unicode_width::UnicodeWidthStr;

use crate::libgit::Libgit;
use crate::note::NoteLedger;
use crate::path::Paths;
use crate::repository::NoteRepository;

pub trait NoteArgs {
    /// Collection of paths to resolve file paths inside.
    /// Used for every operations(add, read, edit, delete)
    fn paths(&self) -> &Paths;
    /// line number user inputs which starts from 1 which differs from system line number
    /// Used only note add, edit and delete
    fn user_line(&self) -> usize;
    /// line number system uses which starts from 0 which differs from user line number
    /// Used only note add, edit and delete
    fn sys_line(&self) -> usize;
    /// message user inputs
    fn message(&self) -> String;
}

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

    pub fn add_note<A>(&self, args: &A) -> anyhow::Result<()>
    where
        A: NoteArgs,
    {
        let ledger = self.note_repository.read_note(args.paths())?;
        if ledger.opaque_exists(args.sys_line()) {
            return Err(anyhow!("comment already exists for line {} in {}. consider to use `edit` instead.", args.user_line(), args.paths()));
        }
        ledger.append(args.sys_line(), args.message())?;
        self.note_repository.write_note(args.paths(), &ledger.plain_note())?;
        return Ok(());
    }

    // TODO : replace parameters into `NoteArgs`
    pub fn read_note(&self, paths: &Paths) -> anyhow::Result<NoteLedger<T>> {
        let ledger = self.note_repository.read_note(paths)?;
        return Ok(ledger);
    }

    pub fn edit_note(&self, paths: &Paths, line: usize, message: String) -> anyhow::Result<()> {
        let ledger = self.note_repository.read_note(paths)?;
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
        let ledger = self.note_repository.read_note(paths)?;

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
    use crate::libgit::{Libgit, ProcessLibgit};
    use crate::note::Note;
    use crate::path::{PathResolver, Paths};
    use crate::repository::NoteRepository;
    use crate::testlib::TestRepo;

    struct Sut {
        repo: TestRepo,
        paths: Paths,
        note_handler: NoteHandler<ProcessLibgit<SimilarGitDiffer>>,
    }

    impl Sut {
        fn setup(content: &str) -> anyhow::Result<Self> {
            let repo = TestRepo::new();
            let _ = repo.create_file("test.txt", Some(content))?;
            let libgit = ProcessLibgit::new(SimilarGitDiffer);
            let paths = PathResolver::resolve(repo.path(), "test.txt")?;

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
        sut.note_handler.add_note(&sut.paths, "hello".to_string())?;

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
        sut.note_handler.add_note(&sut.paths, "hello".to_string())?;
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
        sut.note_handler.add_note(&sut.paths, "hello".to_string())?;
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
        sut.note_handler.add_note(&sut.paths, "hello".to_string())?;
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
