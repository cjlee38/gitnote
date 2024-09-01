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

    pub fn eq_trim(&self, other: &str) -> bool {
        self.snippet.trim() == other.trim()
    }
}

pub trait GitDiffer {
    fn diff(&self, old: &String, new: &String, diff_model: &mut DiffModel);
}

pub struct SimilarGitDiffer;

impl GitDiffer for SimilarGitDiffer {
    fn diff(&self, old: &String, new: &String, diff_model: &mut DiffModel) {
        let mut encountered_but_undetermined_yet = false;

        for change in TextDiff::from_lines(old, new).iter_all_changes() {
            let tag = change.tag();

            match (change.old_index(), change.new_index()) {
                (Some(old_line), Some(new_line))
                    if tag == ChangeTag::Equal && old_line == diff_model.line
                => {
                    // has ever not changed, so it is valid.
                    diff_model.line = new_line;
                    return;
                }
                (Some(old_line), None) if old_line == diff_model.line => {
                    // encountered, so delay the determination until new_line
                    encountered_but_undetermined_yet = true;
                    continue;
                }
                (None, Some(new_line)) if encountered_but_undetermined_yet => {
                    // finally, we can determine it is valid or not.
                    return if diff_model.eq_trim(change.value()) {
                        diff_model.line = new_line;
                    } else {
                        diff_model.valid = false;
                    }
                }
                _ => continue
            }
        } // end of while;

        // if not returned while loop, it determined as invalid.
        diff_model.valid = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_same() {
        let old = r#"
            foo
            bar
            baz
        "#.trim_both_ends();
        let new = r#"
            foo
            bar
            baz
        "#.trim_both_ends();

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "bar".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, true);
        assert_eq!(diff_model.line, 1);
    }

    #[test]
    fn test_diff_change() {
        let old = r#"
            foo
            bar
            baz
        "#.trim_both_ends();
        let new = r#"
            foo
            X
            baz
        "#.trim_both_ends();

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "bar".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, false);
    }
    
    #[test]
    fn test_diff_append() {
        let old = r#"
            foo
            foo2
            foo3
            bar
            baz
        "#.trim_both_ends();
        let new = r#"
            foo
            foo2
            foo3
            bar X
            baz
        "#.trim_both_ends();

        let mut diff_model = DiffModel {
            line: 3,
            snippet: "bar".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, false);
    }
    

    #[test]
    fn test_diff_upper_insert() {
        let old = r#"
            foo
            bar
            baz
        "#.trim_both_ends();
        let new = r#"
            foo
            X
            bar
            baz
        "#.trim_both_ends();

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "bar".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, true);
        assert_eq!(diff_model.line, 2);
    }
    
    #[test]
    fn test_diff_lower_insert() {
        let old = r#"
            foo
            bar
            baz
        "#.trim_both_ends();
        let new = r#"
            foo
            bar
            X
            baz
        "#.trim_both_ends();

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "bar".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, true);
        assert_eq!(diff_model.line, 1);
    }
    
    #[test]
    fn test_diff_delete() {
        let old = r#"
            foo
            bar
            baz
        "#.trim_both_ends();
        let new = r#"
            foo
            baz
        "#.trim_both_ends();

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "bar".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, false);
    }
    
    #[test]
    fn test_diff_space() {
        let old = r#"
            foo
            bar
            baz
        "#.trim_both_ends();
        let new = r#"
            foo
                bar
            baz
        "#.trim_both_ends();

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "bar".to_string(),
            valid: true,
        };
        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, true);
        assert_eq!(diff_model.line, 1);
    }

    #[test]
    fn test_diff_multiple() {
        let old = r#"
            foo
            bar
            baz
        "#.trim_both_ends();
        let new = r#"
            Z
            foo
            Y
            bar
            X
            baz
        "#.trim_both_ends();

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "bar".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, true);
        assert_eq!(diff_model.line, 3);
    }

    #[test]
    fn test_diff_remove_all() {
        let old = r#"
            foo
            bar
            baz
        "#.trim_both_ends();
        let new = r#"
            bar
        "#.trim_both_ends();

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "bar".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, true);
        assert_eq!(diff_model.line, 0);
    }

    #[test]
    fn test_diff_from_empty() {
        let old = r#"
            foo

            bar
        "#.trim_both_ends();
        let new = r#"
            foo

            baz
        "#.trim_both_ends();

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "0".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, true);
        assert_eq!(diff_model.line, 1);
    }

    #[test]
    fn test_diff_from_empty_add() {
        let old = r#"
            foo

            bar
        "#.trim_both_ends();
        let new = r#"
            foo
            X
            baz
        "#.trim_both_ends();

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, false);
    }

    #[test]
    fn test_diff_from_empty_empty_add() {
        let old = r#"
            foo

            bar
        "#.trim_both_ends();
        let new = r#"
            foo


            baz
        "#.trim_both_ends();

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "0".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, true);
        assert_eq!(diff_model.line, 1);
    }

    #[test]
    fn tmp() {
        let x = r#"

        1
        2

        "#.trim_both_ends();
        println!("`{}`", x);
    }

    #[test]
    fn test_diff_from_empty_empty_upper_empty() {
        let old = "foo\n\nbar";
        let new = "\nfoo\n\nbar";

        println!("new : `{}`", new);

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "0".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, true);
        assert_eq!(diff_model.line, 2);
    }

    #[test]
    fn test_diff_from_empty_empty_lower_empty() {
        let old = "foo\n\nbar";
        let new = "foo\n\nbar\n\n";

        let mut diff_model = DiffModel {
            line: 1,
            snippet: "0".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, true);
        assert_eq!(diff_model.line, 1);
    }

    #[test]
    fn test_diff_long() {
        let old = r#"
            a
            b
            c
            d
            e
            f
            g
            h
            i
        "#.trim_both_ends();
        let new = r#"
            a
            b
            1
            2
            3
            4
            c
            5
            6
            d
            e
            f
            g
            h
            i
        "#.trim_both_ends();
        let mut diff_model = DiffModel {
            line: 2,
            snippet: "c".to_string(),
            valid: true,
        };

        SimilarGitDiffer.diff(&old.to_string(), &new.to_string(), &mut diff_model);
        assert_eq!(diff_model.valid, true);
        assert_eq!(diff_model.line, 6);
    }
    
    trait TestTrimmer {
        fn trim_both_ends(&self) -> &str;
    }

    impl TestTrimmer for &str {
        fn trim_both_ends(&self) -> &str {
            return self.trim_start_matches('\n').trim_end_matches(' ')
        }
    }
}
