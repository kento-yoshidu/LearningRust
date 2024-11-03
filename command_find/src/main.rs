fn main() {
    if let Err(e) = command_find::get_args().and_then(command_find::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
    println!("Hello, world!");
}
