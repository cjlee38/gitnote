use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::from_utf8;
use std::string::ToString;

use anyhow::{anyhow, Error};
use tempfile::tempdir_in;

use crate::note::Note;

pub struct TestRepo {
    _dir: tempfile::TempDir, // holds tempdir ref to delay cleanup
    path: PathBuf,
}

impl TestRepo {
    pub fn new() -> Self {
        let _dir = tempdir_in(".").unwrap();
        let path = _dir.path().to_path_buf().canonicalize().unwrap();
        let repo = Self { _dir, path };
        repo.command("git init").expect("Failed to initialize git repository");
        repo
    }

    pub fn command(&self, command: &str) -> Result<(), Error> {
        let commands = command.split(" ").collect::<Vec<&str>>();
        println!("commands = {:?}", commands);
        let output = Command::new(commands[0])
            .args(&commands[1..])
            .current_dir(self.path.clone())
            .output()?;
        let success = output.status.success();
        let stdout = if success {
            output.stdout.as_slice()
        } else {
            output.stderr.as_slice()
        };
        let stdout = from_utf8(stdout)?.to_string();
        success.then(|| {
            println!("{}", stdout.clone());
            ()
        }).ok_or(anyhow!(stdout))
    }

    pub fn create_dir(&self, dirname: &str) -> anyhow::Result<PathBuf> {
        let path = self.path.join(dirname);
        std::fs::create_dir(&path)?;
        Ok(path)
    }

    pub fn create_file(&self, filename: &str, content: Option<&str>) -> anyhow::Result<PathBuf> {
        let path = self.path.join(filename);
        let mut file = File::create(&path).expect("Failed to create file");
        if let Some(content) = content {
            file.write_all(content.as_bytes())?;
        }
        Ok(path)
    }

    pub fn read_file(&self, path: &Path) -> anyhow::Result<String> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn read_note(&self, path: &Path) -> anyhow::Result<Note> {
        return Ok(serde_json::from_reader(BufReader::new(File::open(path)?))?);
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

pub trait AnyToString {
    fn str(&self) -> String;
}

impl AnyToString for PathBuf {
    fn str(&self) -> String {
        self.to_str().unwrap().to_string()
    }
}
