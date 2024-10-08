use colored::Colorize;
use unicode_width::UnicodeWidthStr;

use crate::cli::argument::{AddArgs, DeleteArgs, EditArgs, ReadArgs};
use crate::handlers::{NoteArgs, NoteHandler};
use crate::libgit::Libgit;
use crate::note::{Message, Note};

pub mod argument;
pub mod config;

pub struct CliCurator<T>
where
    T: Libgit,
{
    note_handler: NoteHandler<T>,
}

impl<T> CliCurator<T>
where
    T: Libgit,
{
    pub fn new(note_handler: NoteHandler<T>) -> Self {
        Self { note_handler }
    }

    pub fn add_note(&self, args: AddArgs) -> anyhow::Result<()> {
        self.note_handler.add_note(&args)?;
        println!(
            "Successfully added comment for `{}` in range `{}`",
            args.paths().relative().display(),
            args.user_line()
        );
        Ok(())
    }

    pub fn read_note(&self, args: ReadArgs) -> anyhow::Result<()> {
        let ledger = self.note_handler.read_note(&args)?;
        let note = ledger.opaque_note();
        if args.formatted {
            let note_str = serde_json::to_string_pretty(&note)?;
            println!("{}", &note_str);
            return Ok(());
        }
        let content = ledger.content()?;
        self.pretty_print(&note, content)?;
        Ok(())
    }

    fn pretty_print(&self, note: &Note, content: String) -> anyhow::Result<()> {
        content.lines()
            .enumerate()
            .for_each(|(line, row)| {
                let message = note.find(line);
                self.pretty_print_row(message, line + 1, row) // starts from 1
            });
        Ok(())
    }

    fn pretty_print_row(&self, message: Option<&Message>, line: usize, row: &str) {
        print!("{} ", line.to_string().yellow());
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
        self.note_handler.edit_note(&args)?;
        println!("Successfully edited comment for `{}` in range `{}`", &args.paths, args.line);
        Ok(())
    }

    pub fn delete_note(&self, args: DeleteArgs) -> anyhow::Result<()> {
        self.note_handler.delete_note(&args)?;
        println!("Successfully deleted comment for `{}` in range `{}`", &args.paths, args.user_line());
        Ok(())
    }
}
