mod argument;
mod handlers;
mod libgit;
mod io;

fn main() {
    let cli = argument::build_cli();
    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            if let Err(e) = handlers::init_notes() {
                eprintln!("Error initializing notes directory: {}", e);
            }
        }
        Some(("add", add_matches)) => {
            if let Err(e) = handlers::add_note(
                add_matches.get_one::<String>("file").expect("required").clone(),
                add_matches.get_one::<String>("line").expect("required").clone(),
                add_matches.get_one::<String>("message").expect("required").clone(),
            ) {
                eprintln!("Error adding note: {}", e);
            }
        }
        Some(("view", view_matches)) => {
            if let Err(e) = handlers::view_notes(
                view_matches.get_one::<String>("file").expect("required").clone()
            ) {
                eprintln!("Error viewing notes: {}", e);
            }
        }
        _ => {}
    }
}
