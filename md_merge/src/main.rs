fn main() {
    if let Err(e) = md_merge::get_args().and_then(md_merge::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
