fn main() {
    if let Err(e) = command_line_find::get_args().and_then(command_line_find::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
