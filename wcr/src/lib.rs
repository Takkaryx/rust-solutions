use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(name = "headr")]
#[command(author = "Takkaryx")]
#[command(version = "1.0")]
#[command(about = "rust head", long_about = None)]
pub struct Cli {
    #[arg(short = 'l', long = "lines", help = "Show line count")]
    lines: bool,
    #[arg(short = 'w', long = "words", help = "Show word count")]
    words: bool,
    #[arg(short = 'c', long = "bytes", help = "Show byte count")]
    bytes: bool,
    #[arg(
        short = 'm',
        long = "chars",
        conflicts_with = "bytes",
        help = "Show character count"
    )]
    chars: bool,
    #[arg(help = "Input files", default_value = "-")]
    files: Vec<String>,
}

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
struct FileInfo {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

impl FileInfo {
    fn print_info(
        &self,
        name: String,
        lines: bool,
        words: bool,
        bytes: bool,
        chars: bool,
    ) -> MyResult<()> {
        let mut printstr: String = String::from("");
        if lines {
            printstr.push_str(&format!("{:8}", self.lines));
        }
        if words {
            printstr.push_str(&format!("{:8}", self.words));
        }
        if bytes {
            printstr.push_str(&format!("{:8}", self.bytes));
        }
        if chars {
            printstr.push_str(&format!("{:8}", self.chars));
        }
        if name != "-" {
            printstr.push_str(&format!(" {}", name));
        }
        println!("{}", printstr);
        Ok(())
    }
}

fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut lines = 0;
    let mut words: usize = 0;
    let mut bytes: usize = 0;
    let mut chars: usize = 0;

    let mut text = String::new();
    loop {
        match file.read_line(&mut text) {
            Ok(0) => {
                if text.trim().is_empty() {
                    break;
                } else {
                    lines += 1;
                }
            }
            Ok(bytes_read) => {
                bytes += bytes_read;
                lines += 1;
                words += text.split_whitespace().count();
                chars += text.chars().count();
            }
            Err(error) => {
                panic!("Error reading line: {}", error);
            }
        }
        text.clear();
    }

    Ok(FileInfo {
        lines,
        words,
        bytes,
        chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expects = FileInfo {
            lines: 1,
            words: 10,
            chars: 48,
            bytes: 48,
        };
        assert_eq!(info.unwrap(), expects);
    }
}

pub fn parse_args() -> Cli {
    let mut args = Cli::parse();
    if [args.lines, args.words, args.bytes, args.chars]
        .iter()
        .all(|v| !v)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }
    args
}

pub fn run(args: Cli) -> MyResult<()> {
    let mut total: FileInfo = FileInfo {
        lines: 0,
        words: 0,
        bytes: 0,
        chars: 0,
    };
    let file_num = args.files.len();
    for filename in args.files.iter() {
        match open(&filename) {
            Err(err) => eprintln!("failed to open {}: {}\n", filename, err),
            Ok(handle) => {
                let info: FileInfo = count(handle)?;
                info.print_info(
                    filename.to_string(),
                    args.lines,
                    args.words,
                    args.bytes,
                    args.chars,
                )?;
                total.lines += info.lines;
                total.words += info.words;
                total.bytes += info.bytes;
                total.chars += info.chars;
            }
        }
    }
    if file_num > 1 {
        total.print_info(
            "total".to_string(),
            args.lines,
            args.words,
            args.bytes,
            args.chars,
        )?;
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
