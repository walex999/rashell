use core::str;
use std::{array, io::stdout};
#[allow(unused_imports)]
use std::io::{self, Write};

fn exit_parsed(input: &Vec<&str>) {
    if input.len() > 1 {
        std::process::exit(input[1].parse::<i32>().unwrap_or(1));
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
    if input.len()>1{
        for func in &input[1..]{
            if functions.contains(func){
                println!("{} is a shell builtin",&func);
            }
            else{
                println!("{}: not found",&func);
            }
        }
    } 
}

fn main() {
    let mut input = String::new();
    loop {
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        stdin.read_line(&mut input).unwrap();
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
