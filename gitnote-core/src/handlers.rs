use anyhow::anyhow;
use colored::Colorize;
use unicode_width::UnicodeWidthStr;

use crate::libgit::Libgit;
use crate::note::NoteLedger;
use crate::path::Paths;
use crate::repository::NoteRepository;

/// Arguments for note operations.
/// Even though this trait can be separated into multiple traits, it is combined into one for simplicity.
/// The constraints should be controlled by [`NoteHandler`] implementation.
pub trait NoteArgs {
    /// Collection of paths to resolve file paths inside.
    ///
    /// Can be used for : add, read, edit, delete
    fn paths(&self) -> &Paths;
    /// line number user inputs which starts from 1 which differs from system line number
    ///
    /// Can be used for : add, edit, delete
    fn user_line(&self) -> usize;
    /// line number system uses which starts from 0 which differs from user line number
    ///
    /// Can be used for : add, edit, delete
    fn sys_line(&self) -> usize;
    /// message user inputs
    ///
    /// /// Can be used for : add, edit
    fn message(&self) -> String;
}

/// A service to handle note operations.
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

    pub fn read_note<A>(&self, args: &A) -> anyhow::Result<NoteLedger<T>>
    where
        A: NoteArgs,
    {
        let ledger = self.note_repository.read_note(args.paths())?;
        return Ok(ledger);
    }

    pub fn edit_note<A>(&self, args: &A) -> anyhow::Result<()>
    where
        A: NoteArgs,
    {
        let paths = args.paths();
        let ledger = self.note_repository.read_note(paths)?;

        return if let Some(uuid) = ledger.opaque_uuid(args.sys_line()) {
            ledger.edit(uuid, args.message());
            self.note_repository.write_note(paths, &ledger.plain_note())?;
            Ok(())
        } else {
            Err(anyhow!("no comment found for line {} in {}. consider to use `add` instead.", args.user_line(), paths))
        };
    }

    pub fn delete_note<A>(&self, args: &A) -> anyhow::Result<()>
    where
        A: NoteArgs,
    {
        let paths = args.paths();
        let ledger = self.note_repository.read_note(paths)?;

        return if let Some(uuid) = ledger.opaque_uuid(args.sys_line()) {
            ledger.delete(uuid);
            self.note_repository.write_note(paths, &ledger.plain_note())?;
            Ok(())
        } else {
            Err(anyhow!("no comment found for line {} in {}", args.user_line(), paths))
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::{Differ, SimilarDiffer};
    use crate::handlers::{NoteArgs, NoteHandler};
    use crate::libgit::{Libgit, ManualLibgit};
    use crate::note::Note;
    use crate::path::{PathResolver, Paths};
    use crate::repository::NoteRepository;
    use crate::testlib::TestRepo;

    struct Sut {
        repo: TestRepo,
        paths: Paths,
        note_handler: NoteHandler<ManualLibgit<SimilarDiffer>>,
    }

    impl Sut {
        fn setup(content: &str) -> anyhow::Result<Self> {
            let repo = TestRepo::new();
            let _ = repo.create_file("test.txt", Some(content))?;
            let libgit = ManualLibgit::new(SimilarDiffer);
            let paths = PathResolver::resolve(repo.path(), "test.txt")?;

            let repository = NoteRepository::new(libgit);
            let note_handler = NoteHandler::new(repository);
            Ok(Sut { repo, paths, note_handler })
        }
    }

    struct TestNoteArgs {
        paths: Paths,
        line: usize,
        message: String,
    }

    impl NoteArgs for TestNoteArgs {
        fn paths(&self) -> &Paths {
            &self.paths
        }

        fn user_line(&self) -> usize {
            self.line
        }

        fn sys_line(&self) -> usize {
            self.line - 1
        }

        fn message(&self) -> String {
            self.message.clone()
        }
    }

    #[test]
    fn test_add_note() -> anyhow::Result<()> {
        // given
        let sut = Sut::setup("foo\nbar\nbaz")?;

        // when
        let args = TestNoteArgs {
            paths: sut.paths.clone(),
            line: 2,
            message: "hello".to_string(),
        };
        sut.note_handler.add_note(&args)?;

        // then
        let note_path = sut.paths.note(&Note::get_id(&sut.paths.relative())?)?;
        assert!(note_path.exists());

        let note: Note = sut.repo.read_note(&note_path)?;
        assert_eq!(note.reference, sut.paths.relative());
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
        let add_args = TestNoteArgs {
            paths: sut.paths.clone(),
            line: 2,
            message: "hello".to_string(),
        };
        sut.note_handler.add_note(&add_args)?;
        let read_args = TestNoteArgs {
            paths: sut.paths.clone(),
            line: 0,
            message: "".to_string(),
        };
        let ledger = sut.note_handler.read_note(&read_args)?;

        // then
        let note = ledger.plain_note();
        assert_eq!(note.reference, sut.paths.relative());
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
        let add_args = TestNoteArgs {
            paths: sut.paths.clone(),
            line: 2,
            message: "hello".to_string(),
        };
        sut.note_handler.add_note(&add_args)?;
        let edit_args = TestNoteArgs {
            paths: sut.paths.clone(),
            line: 2,
            message: "world".to_string(),
        };
        sut.note_handler.edit_note(&edit_args)?;

        // then
        let note_path = sut.paths.note(&Note::get_id(&sut.paths.relative())?)?;
        assert!(note_path.exists());

        let note: Note = sut.repo.read_note(&note_path)?;
        assert_eq!(note.reference, sut.paths.relative());
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
        let add_args = TestNoteArgs {
            paths: sut.paths.clone(),
            line: 2,
            message: "hello".to_string(),
        };
        sut.note_handler.add_note(&add_args)?;
        let delete_args = TestNoteArgs {
            paths: sut.paths.clone(),
            line: 2,
            message: "".to_string(),
        };
        sut.note_handler.delete_note(&delete_args)?;

        // then
        let note_path = sut.paths.note(&Note::get_id(&sut.paths.relative())?)?;
        assert!(note_path.exists());

        let note: Note = sut.repo.read_note(&note_path)?;
        assert_eq!(note.reference, sut.paths.relative());
        assert_eq!(note.messages.len(), 0);

        Ok(())
    }
}
