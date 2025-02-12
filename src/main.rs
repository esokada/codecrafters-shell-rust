#[allow(unused_imports)]
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;
use std::{env, vec};
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

fn print_or_write(message: &str, out_file: Option<&String>) -> () {
    match out_file {
        Some(out_file) => {
            let mut file_to_write = File::create(out_file).expect("problem creating file");
            file_to_write.write(message.as_bytes()).expect("problem writing file");

        },    
        None => println!("{message}")
    }
}

fn execute(exe: &str, parsed_args:&[String], output_file:Option<&String>, error_file:Option<&String>) -> () {
    let builtins = vec!["exit","echo","type","pwd","cd"];

    match exe {
        "exit" if parsed_args[0] == "0" => std::process::exit(0),
        "echo" => {
            let joined = parsed_args.join(" ");
            print_or_write(&format!("{}",joined),output_file);
        }
        "pwd" => {
            let path = env::current_dir().unwrap();
            print_or_write(&format!("{}", path.display()),output_file);
        }
        "cd" if parsed_args[0] == "~" => {
            let key = "HOME";
            let value = env::var(key).unwrap();
            match env::set_current_dir(value) {
                Ok(_) => return,
                Err(_) => print_or_write("couldn't move to home directory", output_file)
            }
        }
        "cd" => match env::set_current_dir(parsed_args[0].clone()) {
            Ok(_) => return,
            Err(_) => print_or_write(&format!("cd: {}: No such file or directory",parsed_args[0]),output_file)
        }
        "type" if builtins.contains(&parsed_args[0].as_str()) => println!("{} is a shell builtin",&parsed_args[0]),
        "type" => match find_executable_in_path(&parsed_args[0]) {
            Some(item) => print_or_write(&format!("{} is {}", parsed_args[0],item.display()), output_file),
            None => print_or_write(&format!("{} not found",parsed_args[0]),output_file)
        },
        command => match find_executable_in_path(command) {
             Some(item) => {
            let parent = item.parent().unwrap();
            let child = item.file_name().unwrap();
            let output = Command::new(child).current_dir(parent).args(parsed_args).output().unwrap();
                        // .expect("failed to run process");
            let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();
            print_or_write(&format!("{}",stdout.trim()), output_file);
            print_or_write(&format!("{}",stderr.trim()), error_file);
            }            
            None => {
                print_or_write(&format!("{}: command not found",command),output_file)
            }
    }
}
}

fn handle_redir(parsed_command:&mut Vec<String>) -> (Vec<String>, Option<&String>, Option<&String>) {
    // handle the writing etc. back in main
    // is initializing variables as None with no type a good practice? 
    let mut output_file = None;
    let mut error_file = None;
    let mut new_command: Vec<String> = Vec::new();
    for i in 0..parsed_command.len() {
        if parsed_command[i] == "1>" || parsed_command[i] == ">" {  
            // make sure there's something after the redirector
            if i+1 < parsed_command.len() {
                output_file = Some(&parsed_command[i+1]);
                break
            }
        }
        else if parsed_command[i] == "2>" {  
            // make sure there's something after the redirector
            if i+1 < parsed_command.len() {
                error_file = Some(&parsed_command[i+1]);
                break
            }
        }
        else {
            new_command.push(parsed_command[i].clone());
        }
    }
    (new_command, output_file, error_file)
}

fn main() {
    loop {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let line = input.trim();
    let mut parsed_command = parse_command(line);

    // let mut out_file: Box<dyn Write> = Box::new(io::stdout());
    // never mind, handle the redirection before executing
    let (parsed_command, output_file, error_file) = handle_redir(&mut parsed_command);
    let exe = parsed_command[0].as_str();
    let parsed_args = &parsed_command[1..];
    //next problem: need to get output from execute() rather than println
    // create the output file (if needed) and pass it into execute
    execute(exe, parsed_args, output_file, error_file);
    // execute in turn gives it to print_or_write
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