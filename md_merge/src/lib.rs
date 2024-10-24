use std::fs::{self, File};
use std::io::{self, Write};
use std::error::Error;

use clap::{App, Arg};
use serde::Deserialize;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Args {
    text: Option<Vec<String>>
}

#[derive(Deserialize, Debug)]
struct Config {
    base_file: String,
    append_file: String,
    output_file: String,
}

fn load_config(file_path: &str) -> MyResult<Config> {
    let config_file = fs::read_to_string(file_path)?;
    let config: Config = serde_yaml::from_str(&config_file)?;

    Ok(config)
}

fn add(base_file: &str, append_file: &str, output_file: &str) -> io::Result<()> {
    let base_content = fs::read_to_string(base_file)?;

    let append_content = fs::read_to_string(append_file)?;

    let mut file = File::create(output_file)?;

    writeln!(file, "{}\n\n{}", base_content, append_content)?;

    Ok(())
}

pub fn get_args() -> MyResult<Args> {
    let matches = App::new("md_merge")
        .version("0.0.1")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
        )
        .get_matches();

    let text = matches.values_of_lossy("text");

    Ok(Args {
        text
    })
}

pub fn run(args: Args) -> MyResult<()> {
    println!("{:?}", args);

    let config_file_path = "config.yml";

    match load_config(config_file_path) {
        Ok(config) => {
            if let Err(e) = add(&config.base_file, &config.append_file, &config.output_file) {
                eprintln!("{e}");
            } else {
                println!("{} の内容に {} を追記して、ファイル {} を作成しました。", config.base_file, config.append_file, config.output_file);
            }
        },
        Err(e) => {
            eprintln!("設定ファイルの読み込みエラー: {}", e);
        }
    }

    /*
    let mdx = "base.md";
    let md = "append.md";

    if let Err(e) = add(mdx, md) {
        eprintln!("{e}");
    } else {
        println!("OK");
    }
    */

    Ok(())
}