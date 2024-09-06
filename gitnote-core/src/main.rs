use clap::Parser;

use gitnote::cli::argument::{CliCommand, CliConfigSubcommand, CliSubcommand};
use gitnote::cli::CliCurator;
use gitnote::diff::SimilarGitDiffer;
use gitnote::handlers::NoteHandler;
use gitnote::libgit::ProcessLibgit;
use gitnote::repository::NoteRepository;

fn main() {
    let libgit = ProcessLibgit::new(SimilarGitDiffer);
    let note_handler = NoteHandler::new(NoteRepository::new(libgit));
    let cli_curator = CliCurator::new(note_handler);
    let cli_command = CliCommand::parse();

    match cli_command.sub {
        CliSubcommand::Add(args) => { cli_curator.add_note(args) }
        CliSubcommand::Read(args) => { cli_curator.read_note(args) }
        CliSubcommand::Edit(args) => { cli_curator.edit_note(args) }
        CliSubcommand::Delete(args) => { cli_curator.delete_note(args) }
        CliSubcommand::Config(config_command) => {
            let sub = match config_command {
                CliConfigSubcommand::Set(args) => { println!("Set, {:?}", args) }
                CliConfigSubcommand::Get(args) => { println!("Get, {:?}", args) }
                CliConfigSubcommand::Show(args) => { println!("Show, {:?}", args) }
                CliConfigSubcommand::Unset(args) => { println!("Unset, {:?}", args) }
            };
            Ok(sub)
        }
    }.unwrap();
}