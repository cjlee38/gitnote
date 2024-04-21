use std::io;
use std::io::BufRead;
use anyhow::anyhow;

pub fn inquire_boolean(prompt: &String) -> anyhow::Result<bool> {
    println!("{}", prompt);
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line)?;
    let lowercase = line.to_lowercase();
    let input = lowercase.as_str().trim();

    return if input == "y" || input == "yes" {
        Ok(true)
    } else if input == "n" || input == "no" {
        Ok(false)
    } else {
        Err(anyhow!("require y(es) or n(o)"))
    }
}

pub fn write_out(output: &String) {
    println!("{}", output);
}
