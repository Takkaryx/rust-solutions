fn main() {
    if let Err(e) = uniqr::parse_args().and_then(uniqr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
