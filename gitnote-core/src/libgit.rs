use std::path::PathBuf;

use git2::{Error, Repository};

#[derive(Debug)]
pub struct GitBlob {
    pub id: String,
    pub content: Vec<String>,
}

pub fn find_root_path() -> PathBuf {
    Repository::discover(".").expect("git repository not found")
        .workdir().expect("git repository working directory not found")
        .to_path_buf()
}

pub fn find_gitnote_path() -> PathBuf {
    let path = find_root_path().join(PathBuf::from(".git/.git-notes"));
    let exist = path.try_exists().expect("failed to find git-note path");
    if !exist {
        std::fs::create_dir(&path).expect("failed to create git-note path");
    }
    return path;
}

pub fn find_git_blob(file_path: &PathBuf) -> Result<GitBlob, Error> {
    let repository = Repository::discover(".")?;

    let head = repository.head()?.resolve()?.peel_to_commit()?;
    let object = head.tree()?.get_path(file_path)?.to_object(&repository)?;

    if let Some(blob) = object.as_blob() {
        let id = blob.id().to_string();
        let content_bytes = blob.content();
        let content_str = std::str::from_utf8(content_bytes) // TODO : What if content is not utf8 ?
            .map_err(|e| Error::from_str(&format!("UTF-8 decoding error: {}", e)))?;
        let content = split_lines(content_str);

        Ok(GitBlob { id, content })
    } else {
        Err(Error::from_str("Not a blob"))
    }
}

fn split_lines(s: &str) -> Vec<String> {
    s.replace("\r\n", "\n").split('\n').map(String::from).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_root_path() {
        let path = find_root_path();
        println!("{:?}", path);
    }

    #[test]
    fn test_find_git_blob() {
        let file_path = PathBuf::from("src/main.rs");
        let option = find_git_blob(&file_path);
        let blob = option.unwrap();
        println!("{:?}", blob);
    }
}