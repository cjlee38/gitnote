use clap::{Arg, Command};

// TODO : edit, config
pub fn build_cli() -> Command {
    Command::new("git-note")
        .version("1.0")
        .author("Your Name <your_email@example.com>")
        .about("Manages personal notes on your git repository")
        .subcommand(
            Command::new("add")
                .about("Adds a new note")
                .arg(Arg::new("file")
                    .short('f')
                    .long("file")
                    .help("Specifies the file to add a note to")
                    .value_name("FILE")
                    .required(true))
                .arg(Arg::new("line")
                    .short('l')
                    .long("line")
                    .help("Specifies the line number to add a note to")
                    .value_name("LINE")
                    .required(true))
                .arg(Arg::new("message")
                    .short('m')
                    .long("message")
                    .help("The note message")
                    .value_name("MESSAGE")
                    .required(true)),
        )
        .subcommand(Command::new("init").about("Initializes the note system for the repository"))
        .subcommand(
            Command::new("view")
                .about("Views notes for a file")
                .arg(Arg::new("file")
                    .short('f')
                    .long("file")
                    .help("Specifies the file to view notes for")
                    .value_name("FILE")
                    .required(true)),
        )
}
