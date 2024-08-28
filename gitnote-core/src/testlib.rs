use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8;
use std::string::ToString;

use anyhow::{anyhow, Error};
use tempfile::tempdir_in;

pub struct TestRepo {
    dir: tempfile::TempDir, // holds tempdir ref to delay cleanup
    path: PathBuf,
}

impl TestRepo {
    pub fn new() -> Self {
        let dir = tempdir_in(".").unwrap();
        let path = dir.path().to_path_buf();
        let repo = Self { dir, path };
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

    pub fn create_file(&self, filename: &str, content: Option<&str>) -> anyhow::Result<PathBuf> {
        let path = self.path.join(filename);
        let mut file = File::create(&path).expect("Failed to create file");
        if let Some(content) = content {
            file.write_all(content.as_bytes())?;
        }
        Ok(path)
    }

    pub fn path_as_string(&self) -> anyhow::Result<String> {
        Ok(self.path.to_str().ok_or(anyhow!("path {:?} to str failed", self.path))?.to_string())
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
