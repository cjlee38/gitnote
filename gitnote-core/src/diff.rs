use similar::{ChangeTag, TextDiff};
use crate::note::Message;

#[derive(Debug)]
pub struct DiffModel {
    pub line: usize,
    pub snippet: String,
    pub valid: bool,
}

impl DiffModel {
    pub fn of(message: &Message) -> Self {
        DiffModel {
            line: message.line,
            snippet: (&message).snippet.to_string(),
            valid: true,
        }
    }
}

pub trait GitDiffer {
    fn diff(&self, old: &String, new: &String, diff_model: &mut DiffModel);
}

pub struct SimilarGitDiffer;

impl GitDiffer for SimilarGitDiffer {
    fn diff(&self, old: &String, new: &String, diff_model: &mut DiffModel) {
        for change in TextDiff::from_lines(old, new).iter_all_changes() {
            let tag = change.tag();
            let old_line = change.old_index().unwrap_or(usize::MAX);
            let new_line = change.new_index().unwrap_or(usize::MAX);

            if old_line == diff_model.line {
                let content = change.value();
                if diff_model.snippet.trim() != content.trim()
                    || (tag == ChangeTag::Delete || tag == ChangeTag::Insert)
                {
                    diff_model.valid = false;
                } else {
                    diff_model.line = new_line;
                }
                break;
            }
        }
    }
}
