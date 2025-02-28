use std::io::stdout;
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
                _ => eprintln!("{}: command not found", input.trim()),
            }
        }
    }
}
