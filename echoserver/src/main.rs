use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let addr = &args[1];
    echo_server(addr)?;
    Ok(())
}

fn echo_server(address: &str) {}
