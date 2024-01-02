fn main() {
    let args = wcr::parse_args();
    if let Err(e) = wcr::run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
