use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,           // 表示する行数
    bytes: Option<usize>,   // 表示するバイト数
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("command_line_head")
        .version("0.1.0")
        .author("kent")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input Files")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Input display lines")
                .default_value("10")
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Number of bytes")
        )
        .get_matches();

    let line = matches
            .value_of("line")
            .map(parse_positive_int)
            .transpose() // Option<Result> からResult<Option>への変換
            .map_err(|e| format!("illigal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: line.unwrap(),
        bytes: bytes,
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}