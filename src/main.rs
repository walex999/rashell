use std::io::stdout;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();
    
        
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        match input{
            _ => eprintln!("{}: command not found",input.trim()),
        }
    }
}   