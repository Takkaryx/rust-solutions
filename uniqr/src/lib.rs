use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Parser, Debug)]
#[command(name = "headr")]
#[command(author = "Takkaryx")]
#[command(version = "1.0")]
#[command(about = "rust head", long_about = None)]
pub struct Cli {
    #[arg(short = 'c', long = "count", help = "Show counts")]
    lines: bool,
    #[arg(help = "Input files", default_value = "-")]
    ifile: String,
    #[arg(help = "Output file")]
    ofile: Option<String>,
}

pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn parse_args() -> MyResult<Cli> {
    let args = Cli::parse();
    Ok(args)
}

pub fn run(args: Cli) -> MyResult<()> {
    let mut file = open(&args.ifile).map_err(|e| format!("{}: {}", args.ifile, e))?;

    let mut out_file: Box<dyn Write> = match &args.ofile {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if args.lines {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };
    let mut text = String::new();
    let mut pre_match = String::new();
    let mut count = 0;
    loop {
        let bytes = file.read_line(&mut text)?;
        if bytes == 0 {
            break;
        }
        if text.trim_end() != pre_match.trim_end() {
            print(count, &pre_match)?;
            pre_match = text.to_string();
            count = 0;
        }
        count += 1;
        text.clear();
    }
    print(count, &pre_match)?;
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
