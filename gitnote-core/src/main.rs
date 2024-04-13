mod argument;
mod handlers;
mod libgit;
mod io;

fn main() {
    let cli = argument::build_cli();
    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("add", add_matches)) => {
            if let Err(e) = handlers::add_note(
                add_matches.get_one::<String>("file").expect("required").clone(),
                add_matches.get_one::<String>("line").expect("required").clone(),
                add_matches.get_one::<String>("message").expect("required").clone(),
            ) {
                eprintln!("Error adding note: {}", e);
            }
        }
        Some(("edit", edit_matches)) => {
            if let Err(e) = handlers::edit_note(
                edit_matches.get_one::<String>("file").expect("required").clone(),
                edit_matches.get_one::<String>("line").expect("required").clone(),
                edit_matches.get_one::<String>("message").expect("required").clone(),
            ) {
                eprintln!("Error editing note: {}", e);
            }
        }
        Some(("read", read_matches)) => {
            if let Err(e) = handlers::read_notes(
                read_matches.get_one::<String>("file").expect("required").clone()
            ) {
                eprintln!("Error viewing notes: {}", e);
            }
        }
        Some(("delete", delete_matches)) => {
            if let Err(e) = handlers::delete_note(
                delete_matches.get_one::<String>("file").expect("required").clone(),
                delete_matches.get_one::<String>("line").expect("required").clone(),
            ) {
                eprintln!("Error deleting note: {}", e);
            }
        }
        _ => {}
    }
}
