use std::path::PathBuf;

use anyhow::{anyhow, Context};

use crate::io::{read_all_note, read_or_create_note, read_valid_note, write_note};
use crate::libgit::{find_git_blob, find_root_path, get_diff, is_file_staged, stage};
use crate::note::Message;
use crate::stdio::{inquire_boolean, write_out};

pub fn add_note(file_name: String, line_expr: String, message: String) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;
    if !is_file_staged(&file_path)? {
        write_out(get_diff(&file_path)?);
        if inquire_boolean(
            &format!("File \"{}\" is not up-to-date. \
            Would you stage the file before adding comment ?(y/n)", &file_name),
        )? {
            stage(&file_path)?;
        }
    }

    let blob = find_git_blob(&file_path)?;
    let (start, end) = parse_line_range(&line_expr)?;

    let mut note = read_or_create_note(&file_path)?;
    let message = Message::new(&blob, start, end, message)?;
    note.append(message)?;
    write_note(&note)?;
    println!(
        "successfully added comment for {:?} in range {}:{}",
        &file_path, start, end
    );
    return Ok(());
}

fn parse_line_range(line_expr: &str) -> anyhow::Result<(usize, usize)> {
    let parts: Vec<&str> = line_expr.split(':').collect();
    match parts.len() {
        1 => {
            let line = parts[0].parse::<usize>()?;
            Ok((line, line))
        }
        2 => {
            let start = parts[0].parse::<usize>()?;
            let end = parts[1].parse::<usize>()?;
            Ok((start, end))
        }
        _ => Err(anyhow!("invalid line range format : {line_expr}")),
    }
}

fn resolve_path(input_path: &String) -> anyhow::Result<PathBuf> {
    let abs_path = PathBuf::from(input_path)
        .canonicalize()
        .with_context(|| format!("cannot find to specified file [{input_path}]."))?;
    let root_path = find_root_path()?;

    if !abs_path.exists() || !abs_path.starts_with(&root_path) {
        return Err(anyhow!(format!(
            "specified file {:?} looks like not contained in git repository of {:?}",
            abs_path, root_path
        )));
    }
    return Ok(abs_path.strip_prefix(&root_path)?.to_path_buf());
}

pub fn read_notes(file_name: String) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;
    let blob = find_git_blob(&file_path)?;
    let note = read_valid_note(&blob.file_path)?;
    let note_str = serde_json::to_string_pretty(&note)?;
    write_out(note_str);
    Ok(())
}

pub fn edit_note(file_name: String, line_expr: String, message: String) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;
    if !is_file_staged(&file_path)? {
        return Err(anyhow!(format!(
            "file \"{}\" is not up-to-date. stage the file using `git add {}` before add comment",
            &file_name, &file_name
        )));
    }
    let blob = find_git_blob(&file_path)?;
    let (start, end) = parse_line_range(&line_expr)?;

    let mut note = read_all_note(&file_path)?;
    let message = Message::new(&blob, start, end, message)?;
    note.edit(message);

    write_note(&note)?;
    return Ok(());
}

pub fn delete_note(file_name: String, line_expr: String) -> anyhow::Result<()> {
    let file_path = resolve_path(&file_name)?;
    let (start, end) = parse_line_range(&line_expr)?;

    let mut note = read_all_note(&file_path)?;
    note.delete(start, end);
    write_note(&note)?;
    return Ok(());
}
