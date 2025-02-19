use std::io::stdout;
#[allow(unused_imports)]
use std::io::{self, Write};


fn exit_parsed(input:&String){
    let parsed: Vec<&str> = input.split_whitespace().collect();
    if parsed.len() > 1 {
       // println!("exiting with code {}",parsed[1].parse::<i32>().unwrap_or(1).to_string());
        std::process::exit(parsed[1].parse::<i32>().unwrap_or(1));
    }
    // println!("exiting with code 1 default");
    std::process::exit(1);
}

fn main() {
    let mut input = String::new();
    loop{
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let stdin = io::stdin();
        stdin.read_line(&mut input).unwrap();

        match input{
            _ if input.starts_with("exit") => exit_parsed(&input),
            _ => eprintln!("{}: command not found",input.trim()),
        }
    }
}   