use std::ops::Deref;
use colored::Colorize;
use unicode_width::UnicodeWidthStr;

use crate::handlers::NoteHandler;
use crate::libgit::{GitBlob, Libgit};
use crate::note::{Message, Note};
use crate::path::PathResolver;
use crate::stdio::write_out;

pub struct Cli<T>
where
    T: Libgit,
{
    note_handler: NoteHandler<T>,
    path_resolver: PathResolver,
}

impl<T> Cli<T>
where
    T: Libgit,
{
    pub fn new(note_handler: NoteHandler<T>, path_resolver: PathResolver) -> Self {
        Self { note_handler, path_resolver }
    }

    pub fn add_note(&self, file_name: String, line: usize, message: String) -> anyhow::Result<()> {
        let paths = self.path_resolver.resolve(&file_name)?;
        self.note_handler.add_note(&paths, line - 1, message)?;
        println!(
            "Successfully added comment for {:?} in range {}",
            &file_name,
            line
        );
        Ok(())
    }

    pub fn read_note(&self, file_name: String, formatted: bool) -> anyhow::Result<()> {
        let paths = self.path_resolver.resolve(&file_name)?;
        let ledger = self.note_handler.read_note(&paths)?;
        // TODO : This is a temporary solution to provide a formatted output
        //      for the note. This should be replaced when JNI is implemented.
        let note = ledger.opaque_note();
        if formatted {
            let note_str = serde_json::to_string_pretty(&note)?;
            write_out(&note_str);
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

    pub fn edit_note(&self, file_name: String, line: usize, message: String) -> anyhow::Result<()> {
        let line = line - 1;

        let paths = self.path_resolver.resolve(&file_name)?;
        self.note_handler.edit_note(&paths, line, message)?;
        println!(
            "Successfully edited comment for {:?} in range {}",
            &file_name,
            line + 1
        );
        Ok(())
    }

    pub fn delete_note(&self, file_name: String, line: usize) -> anyhow::Result<()> {
        let line = line - 1;

        let paths = self.path_resolver.resolve(&file_name)?;
        self.note_handler.delete_note(&paths, line)?;
        write_out(&format!(
            "Successfully deleted comment for {:?} in range {}",
            &file_name,
            line + 1
        ));
        Ok(())
    }
}
