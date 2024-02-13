use crate::Extract::*;
use clap;
use clap::Parser;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Range,
};

pub type MyResult<T> = Result<T, String>;
type PositionList = Vec<Range<usize>>;

#[derive(Debug, Clone)]
enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Parser, Debug)]
#[command(name = "cutr")]
#[command(author = "Takkaryx")]
#[command(version = "1.0")]
#[command(about = "rust cut", long_about = None)]
pub struct Cli {
    #[arg(short = 'b', long = "bytes", help = "selected bytes", value_parser = pos_parser, conflicts_with = "chars_range", conflicts_with = "field_range")]
    bytes_range: Option<PositionList>,
    #[arg(short = 'c', long = "chars", help = "selected chars", value_parser = pos_parser, conflicts_with = "bytes_range", conflicts_with = "field_range")]
    chars_range: Option<PositionList>,
    #[arg(short = 'd', long = "delim", help = "delimiter", default_value = "\t")]
    delimiter: char,
    #[arg(short = 'f', long = "fields", help = "selected fields", value_parser = pos_parser, conflicts_with = "bytes_range", conflicts_with = "chars_range")]
    field_range: Option<PositionList>,
    #[arg(help = "files", default_value = "-")]
    paths: Vec<String>,
}

fn pos_parser(s: &str) -> Result<PositionList, String> {
    let re = Regex::new(r"^\d+$").map_err(|e| e.to_string())?;

    let parse_num = |s: &str| -> MyResult<usize> {
        if re.is_match(s) {
            s.parse::<usize>().map_err(|e| e.to_string())
        } else {
            Err(format!("Invalid number: {}", s).into())
        }
    };

    s.split(',')
        .map(|part| {
            let parts: Vec<&str> = part.split('-').collect();
            match parts.len() {
                1 => {
                    let num = parse_num(parts[0])?;
                    if num == 0 {
                        return Err(format!("Invalid number, must be greater than 0"));
                    }
                    Ok(num - 1..num)
                }
                2 => {
                    let start = parse_num(parts[0])?;
                    let end = parse_num(parts[1])?;
                    if start >= end {
                        return Err(format!(
                            "First number in range ({}) must be lower than second number ({})",
                            start, end
                        ));
                    }
                    if start == 0 {
                        return Err(format!("Invalid range, start must be greater than 0"));
                    }
                    Ok(start - 1..end)
                }
                _ => return Err(format!("Invalid range: {:?}", parts)),
            }
        })
        .collect()
}

pub fn get_args() -> MyResult<Cli> {
    let args = Cli::parse();
    Ok(args)
}

pub fn run(args: Cli) -> MyResult<()> {
    let extract: Extract = match (args.bytes_range, args.chars_range, args.field_range) {
        (Some(val), None, None) => Bytes(val),
        (None, Some(val), None) => Chars(val),
        (None, None, Some(val)) => Fields(val),
        _ => return Err(From::from("Must have --fields, --bytes, or --chars")),
    };

    for filename in args.paths {
        match open(filename) {
            Err(err) => eprintln!("{}, {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[cfg(test)]
mod unit_tests {
    use super::pos_parser;

    #[test]
    fn test_pos_parser() {
        //empty string should error
        assert!(pos_parser(" ").is_err());

        // Zero range is an error
        let res = pos_parser("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "Invalid number, must be greater than 0"
        );

        let res = pos_parser("0-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "Invalid range, start must be greater than 0"
        );

        let res = pos_parser("+1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid number: +1");

        let res = pos_parser("+1-2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid number: +1");

        let res = pos_parser("1-+2");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid number: +2");

        let res = pos_parser("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid number: a");

        let res = pos_parser("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid number: a");

        let res = pos_parser("1-a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid number: a");

        let res = pos_parser("a-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid number: a");

        let res = pos_parser("-");
        assert!(res.is_err());

        let res = pos_parser(",");
        assert!(res.is_err());

        let res = pos_parser("1,");
        assert!(res.is_err());

        let res = pos_parser("1-");
        assert!(res.is_err());

        let res = pos_parser("1-1-1");
        assert!(res.is_err());

        let res = pos_parser("1-1-a");
        assert!(res.is_err());

        let res = pos_parser("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)",
        );

        let res = pos_parser("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)",
        );

        let res = pos_parser("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = pos_parser("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = pos_parser("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = pos_parser("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = pos_parser("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = pos_parser("001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = pos_parser("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = pos_parser("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }
}
