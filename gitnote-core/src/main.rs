use std::env;
use std::process::ExitCode;

use clap::Parser;

use crate::argument::{CliCommand, CliSubCommand};
use crate::cli::CliNoteHandler;
use crate::diff::SimilarGitDiffer;
use crate::handlers::NoteHandler;
use crate::libgit::ProcessLibgit;
use crate::path::PathResolver;
use crate::repository::NoteRepository;

mod argument;
mod handlers;
mod libgit;
mod repository;
mod stdio;
mod note;
mod utils;
mod cli;
mod path;
#[cfg(test)]
mod testlib;
mod diff;

struct Cli;

impl Cli {
    fn handle_note(&self, subcommand: CliSubCommand) {

    }
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    let libgit = ProcessLibgit::new(SimilarGitDiffer);
    let path_resolver = PathResolver::from_input(&current_dir, &libgit).unwrap();
    let note_handler = NoteHandler::new(NoteRepository::new(libgit));
    let cli_note_handler = CliNoteHandler::new(note_handler, path_resolver);
    let cli_command = CliCommand::parse();

    match cli_command.sub {
        CliSubCommand::Add(args) => { cli_note_handler.add_note(args)}
        CliSubCommand::Read(args) => { cli_note_handler.read_note(args)}
        CliSubCommand::Edit(args) => { cli_note_handler.edit_note(args)}
        CliSubCommand::Delete(args) => { cli_note_handler.delete_note(args)}
        CliSubCommand::Clean => {cli_note_handler.clean_notes() }
    }.unwrap();
}
