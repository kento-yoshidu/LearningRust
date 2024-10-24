use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    text: Option<Vec<String>>
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
    Ok(())
}