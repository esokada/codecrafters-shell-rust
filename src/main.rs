#[allow(unused_imports)]
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command;
use std::{env, vec};
use pathsearch::find_executable_in_path;
// use rustyline::completion::Completer;
// use rustyline::Helper;
// use rustyline::Editor;
// use rustyline::hint::Hinter;
// use rustyline::{Completer, Helper, Hinter, Validator};
// use rustyline::{Editor, Highlighter};
use rustyline::completion::Completer;
use rustyline::Config;
use rustyline::config::BellStyle;
use rustyline::Editor;
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

struct MyCompleter {}

// impl MyCompleter {
// }

#[derive(Helper, Hinter, Validator, Highlighter)]
struct MyHelper {
    //file_competer: FilenameCompleter,
    builtin_completer: MyCompleter
}

impl Completer for MyHelper {
    type Candidate = String;

    fn complete(
            &self,
            line: &str,
            _pos: usize,
            _ctx: &rustyline::Context<'_>,
        ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        //TODO: move builtins somewhere else
        //first try builtins
        let builtins = ["exit","echo","type","pwd","cd"];
        let mut completions:Vec<String> = builtins.iter().filter(|w| w.starts_with(line)).map(|s| s.to_string() + " ").collect();
        //if we don't find builtins, try executables in PATH
        if completions.len() == 0 {
            for path in std::env::var("PATH").unwrap().split(":") {
                match fs::read_dir(path) {
                    Ok(items) => {
                        for item in items {
                            let exe_name = item.unwrap().file_name();
                            if exe_name.to_string_lossy().starts_with(line) {
                                let name_string = exe_name.to_string_lossy();
                                let mut name_with_space = name_string.into_owned();
                                name_with_space.push(' ');
                                completions.push(name_with_space);
                            }
                        }

                    },
                    Err(_) => continue
                }
            }
        }
        Ok((0,completions))
    }
}

enum WriterAction {
    Write,
    Append,
    Print,
    // NoAction
}

struct Writer {
    action: WriterAction,
    out_file: Option<String>,
}

impl Writer {
    fn do_write(&self, message: &str) -> () {
        match self.action {
            WriterAction::Write => {
                if let Some(file_path) = &self.out_file {
                    let mut file_to_write = File::create(file_path)
                        .expect("problem creating file");
                    if message.len() > 0 {
                    writeln!(file_to_write,"{}",message)
                    // file_to_write.write(message.as_bytes())
                        .expect("problem writing file")
                    }
                }
            },
            WriterAction::Append => {
                if let Some(file_path) = &self.out_file {
                    let mut file_to_app = OpenOptions::new().append(true).create(true).open(file_path).expect("problem opening file");
                    if message.len() > 0 {
                    writeln!(file_to_app,"{}",message).expect("problem appending to file");
                    // file_to_app.write_all(message.as_bytes()).expect("problem appending to file");
                    }
                }
            },
            WriterAction::Print => if message.len() > 0 {
                println!("{}",message)
            }
            // WriterAction::NoAction => ()
        }
    }
}

fn handle_redir(parsed_command:&mut Vec<String>) -> (Vec<String>, Writer, Writer) {
    let mut output_writer = Writer{action: WriterAction::Print, out_file: None};
    let mut error_writer = Writer{action: WriterAction::Print, out_file: None};
    let mut new_command: Vec<String> = Vec::new();
    for i in 0..parsed_command.len() {
        if parsed_command[i] == "1>" || parsed_command[i] == ">" {  
            // make sure there's something after the redirector
            if i+1 < parsed_command.len() {
                output_writer = Writer{action:WriterAction::Write,out_file: Some(parsed_command[i+1].clone())};
                break
            }
        }
        else if parsed_command[i] == "2>" {  
            // make sure there's something after the redirector
            if i+1 < parsed_command.len() {
                error_writer = Writer{action:WriterAction::Write,out_file: Some(parsed_command[i+1].clone())};
                break
            }
        }
        else if parsed_command[i] == ">>" || parsed_command[i] == "1>>" {
            if i+1 < parsed_command.len() {
                output_writer = Writer{action:WriterAction::Append,out_file: Some(parsed_command[i+1].clone())};
                break
            }
        }
        else if parsed_command[i] == "2>>" {
            if i+1 < parsed_command.len() {
                error_writer = Writer{action:WriterAction::Append,out_file: Some(parsed_command[i+1].clone())};
                break
            }
        }
        else {
            new_command.push(parsed_command[i].clone());
        }
    }
    return (new_command, output_writer, error_writer);
}

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



fn execute(exe: &str, parsed_args:&[String], output_writer:Writer, error_writer:Writer) -> () {
    let builtins = vec!["exit","echo","type","pwd","cd"];

    match exe {
        "exit" if parsed_args[0] == "0" => std::process::exit(0),
        "echo" => {
            let joined = parsed_args.join(" ");
            // print_or_write(&format!("{}",joined),output_file);
            output_writer.do_write(&format!("{}",joined));
            // PROBLEM: we only want to write the empty stderr when we're redirecting
            // (we don't want to print an empty line to terminal)
            // TEMPORARY FIX: check if the error_writer object has an out file
            if error_writer.out_file.is_some() {
                error_writer.do_write("");
            }
            // match error_file {
            //     Some (_) => print_or_write(&"",error_file),
            //     None => ()
            // }   
            },      
        "pwd" => {
            let path = env::current_dir().unwrap();
            output_writer.do_write(&format!("{}", path.display()));
        }
        "cd" if parsed_args[0] == "~" => {
            let key = "HOME";
            let value = env::var(key).unwrap();
            match env::set_current_dir(value) {
                Ok(_) => return,
                Err(_) => error_writer.do_write("couldn't move to home directory")
            }
        }
        "cd" => match env::set_current_dir(parsed_args[0].clone()) {
            Ok(_) => return,
            Err(_) => error_writer.do_write(&format!("cd: {}: No such file or directory",parsed_args[0]))
        }
        "type" if builtins.contains(&parsed_args[0].as_str()) => output_writer.do_write(&format!("{} is a shell builtin",&parsed_args[0])),
        "type" => match find_executable_in_path(&parsed_args[0]) {
            Some(item) => output_writer.do_write(&format!("{} is {}", parsed_args[0],item.display())),
            None => error_writer.do_write(&format!("{} not found",parsed_args[0]))
        },
        command => match find_executable_in_path(command) {
             Some(item) => {
            // let parent = item.parent().unwrap();
            let child = item.file_name().unwrap();
            let curr_dir = env::current_dir().unwrap();
            let output = Command::new(child).current_dir(curr_dir).args(parsed_args).output().unwrap();
                        // .expect("failed to run process");
            let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();
            //TODO: check whether output file exists instead
            // if stdout.len() > 0 {
            output_writer.do_write(&format!("{}",stdout.trim()));
            // }
            // if stderr.len() > 0 {
            error_writer.do_write(&format!("{}",stderr.trim()));
            // }
            }            
            None => {
                error_writer.do_write(&format!("{}: command not found",command))
            }
    }
}
}



fn main() {
    // let completer = MyCompleter {};
    // let helper = MyHelper {builtin_completer};
    // let mut rl = Editor::new().unwrap();
    let config = Config::builder().bell_style(BellStyle::Audible).build();
    let helper = MyHelper {
        builtin_completer: MyCompleter {}
    };
    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(helper));
    loop {
    // keep this
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    //this gets handled by rustyline now
    // let stdin = io::stdin();
    // let mut input = String::new();
    // stdin.read_line(&mut input).unwrap();
    let readline = rl.readline("$ ");
    let input = match readline {
        Ok(line) => line,
        Err(err) => {
        println!("Error reading line with rustyline: {:?}", err);
        break;
        }
    };
    let line = input.trim();
    let mut parsed_command = parse_command(line);
    let (parsed_command, output_writer, error_writer) = handle_redir(&mut parsed_command);
    let exe = parsed_command[0].as_str();
    let parsed_args = &parsed_command[1..];
    execute(exe, parsed_args, output_writer, error_writer);
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