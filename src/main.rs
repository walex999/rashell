use core::str;
#[allow(unused_imports)]
use std::io::{self, Write};

fn exit_parsed(input: &Vec<&str>) {
    if input.len() > 1 {
        std::process::exit(input[1].parse::<i32>().unwrap_or_else(|_| 1));
    }
    std::process::exit(1)
}

fn echo(input: &Vec<&str>){
    if input.len()>1{
        for word in &input[1..] {
            print!("{} ",word);
        }
    }
    println!("");
}

fn r#type(input: &Vec<&str>){
    let functions: Vec<&str> = vec!["echo","exit","type"];
    let paths: Vec<String> = std::env::var("PATH").unwrap_or_default().split(":").map(|s|{s.to_string()}).collect();
    if input.len()>1{
        for func in &input[1..]{
            if functions.contains(func){
                println!("{} is a shell builtin",&func);
            }
            else{
                let mut found: bool = false;

                for path in &paths {
                    let file_path = std::path::Path::new(&path).join(&func);
                    if file_path.exists() {
                        println!("{} is {}/{}", func, path, func);
                        found = true;
                        break;
                    }
                }

                if !found {
                    println!("{}: not found", func);
                }            
        
            }
        }
    } 
}

fn main() {
    let mut input = String::new();
    loop {
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap_or_default();

        let stdin = io::stdin();
        stdin.read_line(&mut input).unwrap_or_default();
        let parsed: Vec<&str> = input.split_whitespace().collect();
        if parsed.len() > 0 {
            match parsed[0] {
                "exit" => exit_parsed(&parsed),
                "echo" => echo(&parsed),
                "type" => r#type(&parsed),
                _ => eprintln!("{}: command not found", input.trim()),
            }
        }
    }
}
