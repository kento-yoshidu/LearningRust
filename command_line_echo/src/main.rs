use clap::{App, Arg};

fn main() {
    let temp =
        App::new("echor")
            .version("0.1")
            .author("kent")
            .about("Rust echo")
            .arg(
                Arg::with_name("text")
                    .value_name("TEXT")
                    .help("Input any text")
                    .required(true)
                    .min_values(1),
            )
            .arg(
                Arg::with_name("omit_newline")
                    .short("n")
                    .help("Do not print newline")
                    .takes_value(false),
            )
            .get_matches();

    let text = temp.values_of_lossy("text").unwrap();
    let omit_newline = temp.is_present("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
