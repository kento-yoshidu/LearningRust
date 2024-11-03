use crate::EntryType::*;
use clap::{App, Arg};
// use predicates::path;
use regex::Regex;
use walkdir::WalkDir;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("command_find")
        .version("0.1.0")
        .author("Kento Yoshizu")
        .about("Rust find")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("search path")
                .default_value(".")
                .multiple(true),
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .short("n")
                .long("name")
                .help("Name")
                .takes_value(true)
                .multiple(true)
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .help("Entry type")
                .possible_values(&["f", "d", "l"])
                .multiple(true)
        )
        .get_matches();

    let names = matches
        .values_of_lossy("names")
        .map(|values| {
            values.into_iter()
                .map(|name| {
                    Regex::new(&name)
                        .map_err(|_| {
                            format!("Invalid --name \"{}\"", name)
                        })
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    let entry_types = matches
        .values_of_lossy("types")
        .map(|values| {
            values.iter()
                .map(|value| {
                    match value.as_str() {
                        "d" => Dir,
                        "f" => File,
                        "l" => Link,
                        _ => unreachable!("Invalid type"),
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    println!("{:?}, {:?}, {:?}", matches.values_of_lossy("paths"), names, entry_types);

    Ok(Config {
        paths: matches.values_of_lossy("paths").unwrap(),
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{e}"),
                Ok(entry) => println!("{}", entry.path().display()),
            }
        }
    }

    Ok(())
}
