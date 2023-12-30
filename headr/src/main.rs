use clap::Parser;

fn main() {
    let args = handle_args();
    if let Err(e) = headr::run(args.expect("illegal value in args")) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn handle_args() -> headr::MyResult<headr::Cli> {
    let args = headr::Cli::parse();
    let lines = args.lines;
    match args.bytes {
        Some(n) if n <= 0 => {
            return Err(Box::<dyn std::error::Error>::from(format!(
                "illegal bytes value {:?}",
                args.bytes
            )));
        }
        _ => {}
    }
    if lines <= 0 {
        return Err(Box::<dyn std::error::Error>::from(format!(
            "illegal lines value {:?}",
            lines
        )));
    }
    Ok(args)
}
