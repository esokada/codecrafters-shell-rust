#[allow(unused_imports)]
use std::io::{self, Write};
use pathsearch::find_executable_in_path;


fn main() {
    // Uncomment this block to pass the first stage
    loop {
    print!("$ ");
    io::stdout().flush().unwrap();

    //naive
    // let path = env::var("PATH").unwrap();
    // println!("{:?}",path);

    //using env::split_paths
    // let key = "PATH";
    // match env::var_os(key) {
    //     Some(paths) => {
    //         let collected_paths:Vec<std::path::PathBuf> = env::split_paths(&paths).collect();
    //         // for path in env::split_paths(&paths) {
    //         //     println!("'{}'", path.display());
    //         // }
    //     }
    //     None => println!("{key} is not defined in the environment.")
    // }


    // define builtins
    let builtins = vec!["exit","echo","type"];
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let line = input.trim();
    let line_vec: Vec<&str> = line.splitn(2," ").collect();
    let command = line_vec[0];

    match command {
        "exit" if line_vec[1] == "0" => std::process::exit(0),
        "echo" => println!("{}",line_vec[1]),
        "type" if builtins.contains(&line_vec[1]) => println!("{} is a shell builtin",line_vec[1]),
        "type" => match find_executable_in_path(line_vec[1]) {
            Some(item) => println!("{} is {}", line_vec[1],item.display()),
            None => println!("{} not found",line_vec[1])
        },
        _ => println!("{}: command not found",line)
    }
}
}


    // match input.trim() {
    //     "exit 0" => std::process::exit(0),
    //     input if input.trim().starts_with("echo") => 
    //     println!("{}",input.trim().replace("echo ","")),
    //     input if input.trim().starts_with("type") => 
    //     // old code for "is a shell builtin"
    //     match input.trim().replace("type ","").as_str()
    //         {
    //             todo!();
    //         }
    //         // {
    //         //     "exit" | "echo" | "type" => println!("{} is a shell builtin",input.trim().replace("type ","")),
    //         //     _ => println!("{}: not found",input.trim().replace("type ",""))
    //         // },
    //         input => println!("{}: command not found",input.trim())
    //         }
    // }
    
