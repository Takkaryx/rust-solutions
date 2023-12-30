use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Parser, Debug)]
#[command(name = "headr")]
#[command(author = "Takkaryx")]
#[command(version = "1.0")]
#[command(about = "rust head", long_about = None)]
pub struct Cli {
    #[arg(
        short = 'n',
        long = "lines",
        help = "num lines to print",
        default_value = "10"
    )]
    pub lines: usize,
    #[arg(
        short = 'c',
        long = "bytes",
        help = "num bytes to print",
        conflicts_with = "lines"
    )]
    pub bytes: Option<usize>,
    #[arg(help = "Input files", default_value = "-")]
    pub files: Vec<String>,
}
pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(args: Cli) -> MyResult<()> {
    let num_files = args.files.len();
    for (file_num, filename) in args.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("failed to open {}: {}\n", filename, err),
            Ok(handle) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if args.bytes.is_some() {
                    print_bytes(handle, args.bytes.unwrap())?;
                } else {
                    print_lines(handle, args.lines)?;
                }
            }
        }
    }
    Ok(())
}

fn print_bytes(handle: Box<dyn BufRead>, bytes: usize) -> MyResult<()> {
    let mut file = handle.take(bytes as u64);
    let mut buf = vec![0u8; bytes];
    let bytes_read = file.read(&mut buf)?;
    print!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
    Ok(())
}

fn print_lines(mut handle: Box<dyn BufRead>, num_lines: usize) -> MyResult<()> {
    let mut line = String::new();
    for _ in 0..num_lines {
        let bytes = handle.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        print!("{}", line);
        line.clear();
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
