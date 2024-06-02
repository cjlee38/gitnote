use std::path::PathBuf;

use anyhow::{anyhow, Context};
use colored::Colorize;
use unicode_width::UnicodeWidthStr;

use crate::io::{read_actual_note, read_or_create_note, read_opaque_note, write_note};
use crate::libgit::{find_root_path, find_volatile_git_blob, stage_file};
use crate::note::Message;
use crate::stdio::write_out;

pub fn add_note(file_name: String, line: usize, message: String) -> anyhow::Result<()> {
    let line = line - 1;
    let file_path = resolve_path(&file_name)?;
    validate_file_staged(&file_path)?;

    let mut note = read_or_create_note(&file_path)?;
    let opaque_note = read_opaque_note(&file_path)?;
    if opaque_note.find_message_indexed(line).is_some() {
        return Err(anyhow!(format!(
            "comment already exists for line {} in {:?}. consider to use `edit` instead.", line + 1, &file_path
        )));
    }

    let blob = find_volatile_git_blob(&file_path)?;
    let message = Message::new(&blob, line, message)?;
    note.append(message)?;
    write_note(&note)?;
    write_out(&format!("Successfully added comment for {:?} in range {}",
                      &file_path, line + 1));
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

    let blob = find_volatile_git_blob(&file_path)?;

    let note = read_opaque_note(&file_path)?;
    let content = &blob.content;

    if formatted {
        let note_str = serde_json::to_string_pretty(&note)?;
        write_out(&note_str);
        return Ok(());
    }
    let messages = note.messages;
    content.iter().enumerate()
        .for_each(|(line, line_content)| {
            let message = messages.iter().rev().find(|m| m.line == line);
            if let Some(found) = message {
                let message_lines = found.message.split("\n")
                    .map(String::from)
                    .collect::<Vec<String>>();
                let padding = found.snippet.width();
                for (i, line) in message_lines.iter().enumerate() {
                    if i == 0 {
                        write_out(&format!("{} {} {} ",
                                           (found.line + 1).to_string().yellow(),
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
    let line = line - 1;
    let file_path = resolve_path(&file_name)?;
    validate_file_staged(&file_path)?;

    let mut actual_note = read_actual_note(&file_path)?;
    let opaque_note = read_opaque_note(&file_path)?;

    return if let Some((_, message_found)) = opaque_note.find_message_indexed(line) {
        let uuid = &message_found.uuid;
        for message_to_update in &mut actual_note.messages {
            if message_to_update.uuid == *uuid {
                message_to_update.message = message.clone();
                message_to_update.updated_at = chrono::Utc::now();
            }
        }

        write_note(&actual_note)?;
        Ok(())
    } else {
        Err(anyhow!(format!(
            "no comment found for line {} in {:?}. consider to use `add` instead.", line + 1, &file_path
        )))
    }
}

pub fn delete_note(file_name: String, line: usize) -> anyhow::Result<()> {
    let line = line - 1;
    let file_path = resolve_path(&file_name)?;
    validate_file_staged(&file_path)?;

    let mut actual_note = read_actual_note(&file_path)?;
    let opaque_note = read_opaque_note(&file_path)?;

    return if let Some((_, message_found)) = opaque_note.find_message_indexed(line) {
        let uuid = &message_found.uuid;
        let x: Vec<Message> = actual_note.messages.into_iter()
            .filter(|m| m.uuid != *uuid)
            .collect();
        actual_note.messages = x;

        write_note(&actual_note)?;
        Ok(())
    } else {
        Err(anyhow!(format!("no comment found for line {} in {:?}", line + 1, &file_path)))
    }
}
