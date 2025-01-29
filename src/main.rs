#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use pathsearch::find_executable_in_path;


fn main() {
    // Uncomment this block to pass the first stage
    loop {
    print!("$ ");
    io::stdout().flush().unwrap();

    // define builtins
    let builtins = vec!["exit","echo","type"];
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let line = input.trim();
    let line_vec: Vec<&str> = line.splitn(2," ").collect();
    let command = line_vec[0];
    // problem: need to collect args but handle the case when there are no args
    let args = &line_vec[1..];

    match command {
        "exit" if args[0] == "0" => std::process::exit(0),
        "echo" => println!("{}",args[0]),
        "type" if builtins.contains(&args[0]) => println!("{} is a shell builtin",line_vec[1]),
        "type" => match find_executable_in_path(args[0]) {
            Some(item) => println!("{} is {}", args[0],item.display()),
            None => println!("{} not found",args[0])
        },
        command => match find_executable_in_path(command) {
             Some(item) => {
            // problem: this won't work for 0 or variable numbers of args
             let status = Command::new(item).args(args).status()
            .expect("failed to execute process");
            println!("{:?}",status);
             }
            None => {
                println!("{} not found",command)
            }
        // _ => println!("{}: command not found",line)
    }
}
}
}