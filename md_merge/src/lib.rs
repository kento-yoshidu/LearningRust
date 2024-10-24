use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    text: Option<Vec<String>>
}

fn add(base_file: &str, append_file: &str) -> io::Result<()> {
    let md_content = fs::read_to_string(append_file)?;

    let mut file = OpenOptions::new()
        .append(true)
        .open(base_file)?;

    writeln!(file, "{}", md_content)?;

    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("md_merge")
        .version("0.0.1")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
        )
        .get_matches();

    let text = matches.values_of_lossy("text");

    Ok(Config {
        text
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mdx = "base.md";
    let md = "append.md";

    if let Err(e) = add(mdx, md) {
        eprintln!("{e}");
    } else {
        println!("OK");
    }

    Ok(())
}