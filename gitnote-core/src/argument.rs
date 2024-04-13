use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("git-note")
        .version("1.0")
        .author("cjlee38 <gptpem38@gmail.com>")
        .about("Manages personal notes on your git repository")
        .subcommand(
            Command::new("add")
                .about("Adds a new note")
                .arg(file_argument("Specifies the file to add a note to"))
                .arg(line_argument("Specifies the line number to add a note to"))
                .arg(message_argument("The note message")),
        )
        .subcommand(
            Command::new("read")
                .about("Read notes for a file")
                .arg(file_argument("Specifies the file to view notes for")),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit a note message identified by `--line`")
                .arg(file_argument("Specifies the file to edit a note to"))
                .arg(line_argument("Specifies the line number to edit a note to"))
                .arg(message_argument("Specifies new note message to override previous one")),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a note messaged identified by `--line`")
                .arg(file_argument("Specifies the file to delete"))
                .arg(line_argument("Specifies the line number to delete"))
        )
}

fn file_argument(help_message: &str) -> Arg {
    Arg::new("file")
        .short('f')
        .long("file")
        .help(help_message.to_string())
        .value_name("FILE")
        .required(true)
}

fn line_argument(help_message: &str) -> Arg {
    Arg::new("line")
        .short('l')
        .long("line")
        .help(help_message.to_string())
        .value_name("LINE")
        .required(true)
}

fn message_argument(help_message: &str) -> Arg {
    Arg::new("message")
        .short('m')
        .long("message")
        .help(help_message.to_string())
        .value_name("MESSAGE")
        .required(true)
}
