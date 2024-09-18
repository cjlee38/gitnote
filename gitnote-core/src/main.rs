use clap::Parser;

use gitnote::cli::argument::{CliCommand, CliConfigSubcommand, CliSubcommand};
use gitnote::cli::CliCurator;
use gitnote::cli::config::CliConfig;
use gitnote::diff::SimilarDiffer;
use gitnote::handlers::NoteHandler;
use gitnote::libgit::ManualLibgit;
use gitnote::repository::NoteRepository;

fn main() {
    let libgit = ManualLibgit::new(SimilarDiffer);
    let note_handler = NoteHandler::new(NoteRepository::new(libgit));
    let cli_curator = CliCurator::new(note_handler);
    let cli_command = CliCommand::parse();

    match cli_command.sub {
        CliSubcommand::Add(args) => { cli_curator.add_note(args) }
        CliSubcommand::Read(args) => { cli_curator.read_note(args) }
        CliSubcommand::Edit(args) => { cli_curator.edit_note(args) }
        CliSubcommand::Delete(args) => { cli_curator.delete_note(args) }
        CliSubcommand::Config(config_command) => {
            let cli_config = CliConfig::new();
            let sub = match config_command {
                CliConfigSubcommand::Set(args) => { cli_config.set(args) }
                CliConfigSubcommand::Get(args) => { cli_config.get(args) }
                CliConfigSubcommand::Show(_) => { cli_config.show() }
            }.unwrap();
            Ok(sub)
        }
    }.unwrap();
}