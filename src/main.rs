#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::env;
use pathsearch::find_executable_in_path;

fn parse_args(args: &[&str]) -> Vec<String> {
    let mut result:Vec<String> = Vec::new();
    if args.len() == 0 {
        return result
    }
    let my_args = args[0];
    let mut inside_quote = false;
    let mut current_arg = String::new();

    for c in my_args.chars() {
        if !inside_quote {
            if c != '\'' && c != ' ' {
                current_arg.push(c);
            }         
            else if c == ' ' {
                if !current_arg.is_empty() {
                    result.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            else if c ==  '\'' {
                inside_quote = true;
            }   
        }
        else if inside_quote {
            if c == '\'' {
                inside_quote = false;
                result.push(current_arg.clone());
                current_arg.clear();
            }
            else {
                current_arg.push(c);
            }            
        }
    }
    if !current_arg.is_empty() {
    result.push(current_arg.clone());
    }
    result
}

fn main() {
    loop {
    print!("$ ");
    io::stdout().flush().unwrap();

    // define builtins
    let builtins = vec!["exit","echo","type","pwd","cd"];
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let line = input.trim();
    let line_vec: Vec<&str> = line.splitn(2," ").collect();
    let command = line_vec[0];
    let args = &line_vec[1..];
    let parsed_args = parse_args(args);

    match command {
        "exit" if parsed_args[0] == "0" => std::process::exit(0),
        "echo" => {
            let joined = parsed_args.join(" ");
            println!("{}",joined);
        }
        "pwd" => {
            let path = env::current_dir().unwrap();
            println!("{}",path.display());
        }
        "cd" if parsed_args[0] == "~" => {
            let key = "HOME";
            let value = env::var(key).unwrap();
            match env::set_current_dir(value) {
                Ok(_) => continue,
                Err(_) => println!("couldn't move to home directory")
            }
        }
        "cd" => match env::set_current_dir(parsed_args[0].clone()) {
            Ok(_) => continue,
            Err(_) => println!("cd: {}: No such file or directory",parsed_args[0])
        }
        "type" if builtins.contains(&parsed_args[0].as_str()) => println!("{} is a shell builtin",line_vec[1]),
        "type" => match find_executable_in_path(&parsed_args[0]) {
            Some(item) => println!("{} is {}", parsed_args[0],item.display()),
            None => println!("{} not found",parsed_args[0])
        },
        command => match find_executable_in_path(command) {
             Some(item) => {
            let parent = item.parent().unwrap();
            let child = item.file_name().unwrap();
            let output = Command::new(child).current_dir(parent).args(parsed_args).output().unwrap();
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

#[cfg(test)]
mod tests {
    use super::parse_args;
    #[test]
    fn test_parse_args() {
        assert_eq!(parse_args(&["foo"]), vec!["foo".to_string()]);
        assert_eq!(parse_args(&["foo 'hello world'"]), vec!["foo".to_string(), "hello world".to_string()]);
        assert_eq!(parse_args(&["                          foo 'hello world' bar"]), vec!["foo".to_string(), "hello world".to_string(), "bar".to_string()]);
    }
}