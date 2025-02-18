use std::io::stdout;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    
    print!("$ ");
    io::stdout().flush().unwrap();

    
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input = input.trim().to_string();
    match input{
        _ => eprintln!("{}: command not found",input),
    }
}   