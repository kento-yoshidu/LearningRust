fn main() {
    if let Err(e) = command_line_wc::get_args().and_then(command_line_wc::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
