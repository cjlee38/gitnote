use std::env;
use std::path::Path;
use std::str::FromStr;

use clap::{Args, Parser, Subcommand};

use crate::config::Config;
use crate::handlers::NoteArgs;
use crate::path::{PathResolver, Paths};

#[derive(Debug, Parser)]
pub struct CliCommand {
    #[clap(subcommand)]
    pub sub: CliSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CliSubcommand {
    Add(AddArgs),
    Read(ReadArgs),
    Edit(EditArgs),
    Delete(DeleteArgs),
    #[clap(subcommand)]
    Config(CliConfigSubcommand),
}

/// for clap parser
impl FromStr for Paths {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let current = env::current_dir().unwrap();
        Ok(PathResolver::resolve(&current, s)?)
    }
}

#[derive(Debug, Args)]
pub struct AddArgs {
    #[arg(
        short = 'f',
        long = "file",
        value_name = "FILE",
        help = "Specifies the file to add a note to",
        value_parser = clap::value_parser!(Paths)
    )]
    paths: Paths,
    #[arg(
        short,
        long,
        help = "Specifies the line number to add a note to"
    )]
    line: usize,
    #[arg(
        short,
        long,
        help = "The note message"
    )]
    message: String,
}

impl NoteArgs for AddArgs {
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

#[derive(Debug, Args)]
pub struct ReadArgs {
    #[arg(
        short = 'f',
        long = "file",
        value_name = "FILE",
        help = "Specifies the file to view notes for",
        value_parser = clap::value_parser!(Paths)
    )]
    pub paths: Paths,
    #[arg(
        long,
        help = "Prints the note in a json-formatted way",
        default_value = "false"
    )]
    pub formatted: bool,
}

impl NoteArgs for ReadArgs {
    fn paths(&self) -> &Paths {
        &self.paths
    }

    fn user_line(&self) -> usize {
        unreachable!("user_line is not used in read operation")
    }

    fn sys_line(&self) -> usize {
        unreachable!("sys_line is not used in read operation")
    }

    fn message(&self) -> String {
        unreachable!("message is not used in read operation")
    }
}

#[derive(Debug, Args)]
pub struct EditArgs {
    #[arg(
        short = 'f',
        long = "file",
        value_name = "FILE",
        help = "Specifies the file to edit a note to",
        value_parser = clap::value_parser!(Paths)
    )]
    pub paths: Paths,
    #[arg(
        short,
        long,
        help = "Specifies the line number to edit a note to"
    )]
    pub line: usize,
    #[arg(
        short,
        long,
        help = "Specifies new note message to override previous one"
    )]
    pub message: String,
}

impl NoteArgs for EditArgs {
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

#[derive(Debug, Args)]
pub struct DeleteArgs {
    #[arg(
        short = 'f',
        long = "file",
        value_name = "FILE",
        help = "Specifies the file to delete",
        value_parser = clap::value_parser!(Paths)
    )]
    pub paths: Paths,
    #[arg(
        short,
        long,
        help = "Specifies the line number to delete"
    )]
    pub line: usize,
}

impl NoteArgs for DeleteArgs {
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
        unreachable!("message is not used in delete operation")
    }
}

#[derive(Debug, Subcommand)]
pub enum CliConfigSubcommand {
    Set(ConfigSetArgs),
    Get(ConfigGetArgs),
    Show(ConfigShowArgs),
    Unset(ConfigUnsetArgs),
}

#[derive(Debug, Args)]
pub struct ConfigSetArgs {
    #[arg(short, long, help = "Specifies the key to set")]
    pub key: String,
    #[arg(short, long, help = "Specifies the value to set")]
    pub value: String,
}

#[derive(Debug, Args)]
pub struct ConfigGetArgs {
    #[arg(short, long, help = "Specifies the key to get")]
    pub key: String,
}

#[derive(Debug, Args)]
pub struct ConfigShowArgs {}

#[derive(Debug, Args)]
pub struct ConfigUnsetArgs {
    #[arg(short, long, help = "Specifies the key to unset")]
    pub key: String,
}
