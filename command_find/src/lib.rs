use crate::EntryType::*;
use clap::{App, Arg};
// use predicates::path;
use regex::Regex;
use walkdir::WalkDir;
use std::error::Error;
use walkdir::DirEntry;

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

    Ok(Config {
        paths: matches.values_of_lossy("paths").unwrap(),
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty() ||
        config
            .entry_types
            .iter()
            .any(|entry_type|  match entry_type {
                Link => entry.file_type().is_symlink(),
                Dir => entry.file_type().is_dir(),
                File => entry.file_type().is_file(),
            })
    };

    let name_filter = |entry: &DirEntry| {
        config.names.is_empty() ||
        config
            .names
            .iter()
            .any(|reg| {
                reg.is_match(&entry.file_name().to_string_lossy())
            })
    };

    for path in config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{e}");
                    None
                },
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| {
                entry.path().display().to_string()
            })
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }

    Ok(())
}
