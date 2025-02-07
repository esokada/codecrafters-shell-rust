#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::env;
use pathsearch::find_executable_in_path;

fn parse_command(args: &str) -> Vec<String> {
    let mut result:Vec<String> = Vec::new();
    if args.len() == 0 {
        return result
    }
    // let args = args[0];
    let mut inside_single_quote = false;
    let mut inside_double_quote = false;
    let mut literal = false;
    let mut current_arg = String::new();

    let literals_for_double_quotes = vec!["\\", "$", "\""];
    let newlines = vec!["\\n", "\\r"];

    for (i,c) in args.chars().enumerate() {
        let next_char = args.chars().nth(i+1).unwrap_or_default();
        if !inside_single_quote && !inside_double_quote {
            if literal == true {
                current_arg.push(c);
                literal = false;
                continue
            }
            if c == ' ' {
                if !current_arg.is_empty() {
                    result.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            else if c ==  '\'' {
                inside_single_quote = true;
            }
            else if c == '\"' {
                inside_double_quote = true;
            }        
            else if c == '\\' {
                literal = true;
            }        
            else {
                current_arg.push(c);
                
            }
            }
        
        else if inside_single_quote {
            if c == '\'' && next_char == '\'' {
                inside_single_quote = false;
            }
            else if c == '\'' {
                inside_single_quote = false;
                result.push(current_arg.clone());
                current_arg.clear();
            }
            else {
                current_arg.push(c);
            }            
        }
        else if inside_double_quote {
            if literal == true {
                current_arg.push(c);
                literal = false;
                continue

            }
            //todo: test if next_char in literals_for_double_quotes, newline
            else if c == '\\' && literals_for_double_quotes.contains(&next_char.to_string().as_str()) || newlines.contains(&next_char.to_string().as_str())  {
                literal = true;

            }
            else if c == '\"' && next_char == '\"' {
                inside_double_quote = false;
            }
            else if c == '\"' && next_char == ' '{
                inside_double_quote = false;
                result.push(current_arg.clone());
                current_arg.clear();
            }
            else if c == '\"' {
                continue
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
    // if line.starts_with('\'') || line.starts_with('\"') {
    //     //get quoted exe name
    //     // let quoted_exe_name = parse_args(line);

    // }
    // else case here
    // let line_vec: Vec<&str> = line.splitn(2," ").collect();
    // let command = line_vec[0];
    // let args = &line_vec[1..];
    let parsed_command = parse_command(line);
    let exe = parsed_command[0].as_str();
    let parsed_args = &parsed_command[1..];

    match exe {
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
        "type" if builtins.contains(&parsed_args[0].as_str()) => println!("{} is a shell builtin",exe),
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
    use super::parse_command;
    #[test]
    fn test_parse_args() {
        assert_eq!(parse_command("foo"), vec!["foo".to_string()]);
        assert_eq!(parse_command("\"foo 'hello world'\""), vec!["foo 'hello world'".to_string()]);
        assert_eq!(parse_command("\"foo 'hello world'\" bar"), vec!["foo 'hello world'".to_string(), "bar".to_string()]);
        assert_eq!(parse_command("                          foo 'hello world' bar"), vec!["foo".to_string(), "hello world".to_string(), "bar".to_string()]);
        
    }
}