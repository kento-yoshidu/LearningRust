use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        if arg == "-V" || arg == "--version" {
            println!("example1");
            return;
        }
    }

    loop {
        print!("-> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let line = line.trim();

        if line == "quit" || line == "exit" {
            break;
        }

        if line.to_lowercase() == "quit" {
            println!("Did you mean 'quit'?");
        }

        if line.to_lowercase() == "exit" {
            println!("Did you mean 'exit'?");
        }

        println!("{line}");
    }
}
