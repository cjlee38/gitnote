use std::env;
use std::process::ExitCode;
use crate::handlers::NoteHandler;

mod argument;
mod handlers;
mod libgit;
mod io;
mod stdio;
mod note;
mod utils;
mod cli;
mod path;
#[cfg(test)]
mod testlib;

static EXIT_OK: u8 = 0;
static EXIT_ERR: u8 = 1;

fn handle_command<T>(result: anyhow::Result<T>) -> u8 {
    match result {
        Err(e) => {
            eprintln!("Error: {}", e);
            EXIT_ERR
        }
        Ok(..) => EXIT_OK,
    }
}

fn main() -> ExitCode {
    let cli = argument::build_cli();
    let matches = cli.get_matches();
    let note_handler = NoteHandler::new(&env::current_dir().unwrap());
    let exit_code = match matches.subcommand() {
        Some(("add", add_matches)) => {
            handle_command(note_handler.add_note(
                add_matches.get_one::<String>("file").expect("required").clone(),
                add_matches.get_one::<String>("line").expect("required").parse::<usize>().expect("required"),
                add_matches.get_one::<String>("message").expect("required").clone()
            ))
        }
        Some(("edit", edit_matches)) => {
            handle_command(note_handler.edit_note(
                edit_matches.get_one::<String>("file").expect("required").clone(),
                edit_matches.get_one::<String>("line").expect("required").parse::<usize>().expect("required"),
                edit_matches.get_one::<String>("message").expect("required").clone(),
            ))
        }
        Some(("read", read_matches)) => {
            handle_command(note_handler.read_note(
                read_matches.get_one::<String>("file").expect("required").clone(),
                read_matches.get_flag("format")
            ))
        }
        Some(("delete", delete_matches)) => {
            handle_command(note_handler.delete_note(
                delete_matches.get_one::<String>("file").expect("required").clone(),
                delete_matches.get_one::<String>("line").expect("required").parse::<usize>().expect("required"),
            ))
        }
        e => {
            eprintln!("unknown command : {:?}", e);
            EXIT_ERR
        }
    };
    ExitCode::from(exit_code)
}
