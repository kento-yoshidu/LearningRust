fn main() {
    if let Err(e) = command_line_uniq::get_args().and_then(command_line_uniq::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
