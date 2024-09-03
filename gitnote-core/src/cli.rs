use colored::Colorize;
use unicode_width::UnicodeWidthStr;
use crate::argument::{AddArgs, DeleteArgs, EditArgs, ReadArgs};

use crate::handlers::NoteHandler;
use crate::libgit::{GitBlob, Libgit};
use crate::note::{Message, Note};
use crate::path::PathResolver;
use crate::stdio::{stdin, stdout, stdout_str};

pub struct CliNoteHandler<T>
where
    T: Libgit,
{
    note_handler: NoteHandler<T>,
    path_resolver: PathResolver,
}

impl<T> CliNoteHandler<T>
where
    T: Libgit,
{
    pub fn new(note_handler: NoteHandler<T>, path_resolver: PathResolver) -> Self {
        Self { note_handler, path_resolver }
    }

    pub fn add_note(&self, args: AddArgs) -> anyhow::Result<()> {
        let paths = self.path_resolver.resolve(&args.file)?;
        self.note_handler.add_note(&paths, args.line - 1, args.message)?;
        println!(
            "Successfully added comment for {:?} in range {}",
            &args.file,
            args.line
        );
        Ok(())
    }

    pub fn read_note(&self, args: ReadArgs) -> anyhow::Result<()> {
        let paths = self.path_resolver.resolve(&args.file)?;
        let ledger = self.note_handler.read_note(&paths)?;
        // TODO : This is a temporary solution to provide a formatted output
        //      for the note. This should be replaced when JNI is implemented.
        let note = ledger.opaque_note();
        if args.formatted {
            let note_str = serde_json::to_string_pretty(&note)?;
            println!("{}", &note_str);
            return Ok(());
        }
        let blob = ledger.git_blob()?;
        self.pretty_print(&note, blob)?;
        Ok(())
    }

    fn pretty_print(&self, note: &Note, blob: GitBlob) -> anyhow::Result<()> {
        blob.content.lines()
            .enumerate()
            .for_each(|(line, row)| {
                let message = note.find(line);
                self.pretty_print_row(message, line, row)
            });
        Ok(())
    }

    fn pretty_print_row(&self, message: Option<&Message>, line: usize, row: &str) {
        print!("{} ", (line + 1).to_string().yellow());
        print!("{} ", row);

        match message {
            Some(found) => self.pretty_print_message(row, found),
            None => println!(),
        }
    }

    fn pretty_print_message(&self, row: &str, found: &Message) {
        let padding = row.width();
        let message_lines: Vec<String> = found.message
            .split("\n")
            .map(String::from)
            .collect();
        println!("{}", message_lines[0].red());

        (message_lines.len() > 1)
            .then(|| {
                message_lines.iter()
                    .skip(1)
                    .for_each(|line| println!("{:width$} {}", "", line.red(), width = padding + 2),
                    );
            });
    }

    pub fn edit_note(&self, args: EditArgs) -> anyhow::Result<()> {
        let line = args.line - 1;

        let paths = self.path_resolver.resolve(&args.file)?;
        self.note_handler.edit_note(&paths, line, args.message)?;
        println!(
            "Successfully edited comment for {:?} in range {}",
            &args.file,
            line + 1
        );
        Ok(())
    }

    pub fn delete_note(&self, args: DeleteArgs) -> anyhow::Result<()> {
        let line = args.line - 1;

        let paths = self.path_resolver.resolve(&args.file)?;
        self.note_handler.delete_note(&paths, line)?;
        stdout(&format!(
            "Successfully deleted comment for {:?} in range {}",
            &args.file,
            line + 1
        ));
        Ok(())
    }
}
