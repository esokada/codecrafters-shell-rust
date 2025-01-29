#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    loop {
    print!("$ ");
    io::stdout().flush().unwrap();

    // define commands
    // let commands = vec!["exit","echo","type"];
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    match input.trim() {
        "exit 0" => std::process::exit(0),
        input if input.trim().starts_with("echo") => 
        println!("{}",input.trim().replace("echo ","")),
        input if input.trim().starts_with("type") => 
        match input.trim().replace("type ","").as_str()
            {
                "exit" | "echo" | "type" => println!("{} is a shell builtin",input.trim().replace("type ","")),
                _ => println!("{}: not found",input.trim().replace("type ",""))
            },
            input => println!("{}: command not found",input.trim())
            }
    }
    }
