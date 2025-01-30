#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::env;
use pathsearch::find_executable_in_path;


fn main() {
    loop {
    print!("$ ");
    io::stdout().flush().unwrap();

    // define builtins
    let builtins = vec!["exit","echo","type","pwd"];
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let line = input.trim();
    let line_vec: Vec<&str> = line.splitn(2," ").collect();
    let command = line_vec[0];
    let args = &line_vec[1..];

    match command {
        "exit" if args[0] == "0" => std::process::exit(0),
        "echo" => println!("{}",args[0]),
        "pwd" => {
            let path = env::current_dir().unwrap();
            println!("{}",path.display());
        }
        "type" if builtins.contains(&args[0]) => println!("{} is a shell builtin",line_vec[1]),
        "type" => match find_executable_in_path(args[0]) {
            Some(item) => println!("{} is {}", args[0],item.display()),
            None => println!("{} not found",args[0])
        },
        command => match find_executable_in_path(command) {
             Some(item) => {
            let parent = item.parent().unwrap();
            let child = item.file_name().unwrap();
            let output = Command::new(child).current_dir(parent).args(args).output().unwrap();
                        // .expect("failed to run process");
            let stdout = String::from_utf8(output.stdout).unwrap();
            println!("{}",stdout.trim());
             }
            None => {
                println!("{}: command not found",command)
            }
    }
}
}
}