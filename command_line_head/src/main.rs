fn main() {
    if let Err(e) = command_line_head::get_args().and_then(command_line_head::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
