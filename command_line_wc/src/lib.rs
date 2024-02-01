use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn run(config: Config) -> MyResult<()> {
    for file_name in &config.files {
        match open(file_name) {
            Err(e) => eprintln!("{}: {}", file_name, e),
            Ok(_) => println!("Opened {}", file_name),
        }
    }

    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("command_line_wc")
        .version("0.1.0")
        .author("kent")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true)
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Show line count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Show word count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Show byte count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Show character count")
                .takes_value(false)
                .conflicts_with("bytes")
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|c| c == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I dont want the world";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }
}
