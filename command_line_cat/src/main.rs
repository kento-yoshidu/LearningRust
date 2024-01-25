fn main() {
    if let Err(e) =
        command_line_cat::get_args()
            .and_then(command_line_cat::run) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
}
