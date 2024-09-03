pub fn stdout(output: &String) {
    println!("{}", output);
}

pub fn stdout_str(output: &str) {
    println!("{}", output);
}

pub fn stdin(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
