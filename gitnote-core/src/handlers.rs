use std::path::PathBuf;

use anyhow::{anyhow, Context};
use colored::Colorize;
use unicode_width::UnicodeWidthStr;

use crate::io::{read_all_note, read_or_create_note, read_valid_note, write_note};
use crate::libgit::{find_git_blob, find_root_path, stage_file};
use crate::note::Message;
use crate::stdio::write_out;

pub fn add_note(file_name: String, line: usize, message: String) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;
    validate_file_staged(&file_path)?;

    let blob = find_git_blob(&file_path)?;

    let mut note = read_or_create_note(&file_path)?;
    let message = Message::new(&blob, line, message)?;
    note.append(message)?;
    write_note(&note)?;
    println!(
        "Successfully added comment for {:?} in range {}",
        &file_path, line
    );
    return Ok(());
}

fn validate_file_staged(file_path: &PathBuf) -> anyhow::Result<()> {
    stage_file(&file_path)?;
    return Ok(());
}

fn resolve_path(input_path: &String) -> anyhow::Result<PathBuf> {
    let abs_path = PathBuf::from(input_path)
        .canonicalize()
        .with_context(|| format!("cannot find specified file [{input_path}]."))?;
    let root_path = find_root_path()?;

    if !abs_path.exists() || !abs_path.starts_with(&root_path) {
        return Err(anyhow!(format!(
            "specified file {:?} looks like not contained in git repository of {:?}",
            abs_path, root_path
        )));
    }
    return Ok(abs_path.strip_prefix(&root_path)?.to_path_buf());
}

pub fn read_notes(file_name: String, formatted: bool) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;

    let blob = find_git_blob(&file_path)?;

    let note = read_valid_note(&blob.file_path)?;
    let content = &blob.content;

    if formatted {
        let note_str = serde_json::to_string_pretty(&note)?;
        write_out(&note_str);
        return Ok(());
    }
    let messages = note.messages;
    content.iter().enumerate()
        .for_each(|(line, line_content)| {
            let message = messages.iter().find(|m| m.line == line + 1);
            if let Some(found) = message {
                let message_lines = found.message.split("\n")
                    .map(String::from)
                    .collect::<Vec<String>>();
                let padding = found.snippet.width();
                for (i, line) in message_lines.iter().enumerate() {
                    if i == 0 {
                        write_out(&format!("{} {} {} ",
                                           found.line.to_string().yellow(),
                                           found.snippet,
                                           line.red())
                        );
                    } else {
                        write_out(&format!("{:width$} {}",
                                           "",
                                           line.red(),
                                           width = padding + 2)
                        );
                    }
                }
            } else {
                write_out(&format!("{} {}", (line + 1).to_string().yellow(), line_content));
            }
        });

    Ok(())
}

pub fn edit_note(file_name: String, line: usize, message: String) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;
    validate_file_staged(&file_path)?;

    let blob = find_git_blob(&file_path)?;

    let mut note = read_all_note(&file_path)?;
    let message = Message::new(&blob, line, message)?;
    note.edit(message);

    write_note(&note)?;
    return Ok(());
}

pub fn delete_note(file_name: String, line: usize) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;

    let mut note = read_all_note(&file_path)?;
    note.delete(line);
    write_note(&note)?;
    return Ok(());
}
