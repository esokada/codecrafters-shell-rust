#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    loop {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    if input.trim() == "exit 0".to_string() {
        std::process::exit(0);
    }
    else {
    println!("{}: command not found", input.trim())
    }
}
}
