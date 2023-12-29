use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rat")]
#[command(author = "Takkaryx")]
#[command(version = "1.0")]
#[command(about = "rust cat", long_about = None)]
struct Cli {
    #[arg(short = 'n', long = "number", help = "num lines")]
    number_lines: bool,
    #[arg(short = 'b', long = "number-nonblank", help = "num non-blank lines")]
    number_nonblank: bool,
    #[arg(help = "Input files")]
    files: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    if args.number_lines && args.number_nonblank {
        eprintln!("error: The argument '-n' cannot be used with '-b'");
        std::process::exit(1);
    }
    if let Err(e) = rat::run(args.files, args.number_lines, args.number_nonblank) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
