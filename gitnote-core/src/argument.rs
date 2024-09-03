use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct CliCommand {
    #[clap(subcommand)]
    pub sub: CliSubCommand
}

#[derive(Debug, Subcommand)]
pub enum CliSubCommand {
    Add(AddArgs),
    Read(ReadArgs),
    Edit(EditArgs),
    Delete(DeleteArgs),
}

#[derive(Debug, Args)]
pub struct AddArgs {
    #[arg(short, long, help = "Specifies the file to add a note to")]
    pub file: String,
    #[arg(short, long, help = "Specifies the line number to add a note to")]
    pub line: usize,
    #[arg(short, long, help = "The note message")]
    pub message: String
}

#[derive(Debug, Args)]
pub struct ReadArgs {
    #[arg(short, long, help = "Specifies the file to view notes for")]
    pub file: String,
    #[arg(long, help = "Prints the note in a json-formatted way", default_value = "false")]
    pub formatted: bool
}

#[derive(Debug, Args)]
pub struct EditArgs {
    #[arg(short, long, help = "Specifies the file to edit a note to")]
    pub file: String,
    #[arg(short, long, help = "Specifies the line number to edit a note to")]
    pub line: usize,
    #[arg(short, long, help = "Specifies new note message to override previous one")]
    pub message: String
}

#[derive(Debug, Args)]
pub struct DeleteArgs {
    #[arg(short, long, help = "Specifies the file to delete")]
    pub file: String,
    #[arg(short, long, help = "Specifies the line number to delete")]
    pub line: usize
}

