use clap;
use clap::Parser;
use regex::Regex;
use std::error::Error;
use std::str::FromStr;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(name = "findr")]
#[command(author = "Takkaryx")]
#[command(version = "1.0")]
#[command(about = "rust find", long_about = None)]
pub struct Cli {
    #[arg(num_args(0..), short = 'n', long = "name", help = "Name", value_parser = parse_entry_name)]
    names: Vec<Regex>,
    #[arg(num_args(0..), short = 't', long = "type", help = "Entry type",value_parser = parse_entry_type)]
    entry_types: Vec<EntryType>,
    #[arg(num_args(0..), help = "Path", default_value = ".")]
    paths: Vec<String>,
}

#[derive(clap::ValueEnum, Debug, Eq, PartialEq, Clone, Copy)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl FromStr for EntryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "d" => Ok(EntryType::Dir),
            "f" => Ok(EntryType::File),
            "l" => Ok(EntryType::Link),
            _ => Err(format!("Invalid entry type: {}", s)),
        }
    }
}

fn parse_entry_type(s: &str) -> Result<EntryType, String> {
    s.parse()
}

fn parse_entry_name(s: &str) -> Result<Regex, String> {
    Regex::new(&s).map_err(|_| format!("Invalid --name \"{}\"", s))
}

pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Cli> {
    let args = Cli::parse();
    Ok(args)
}

pub fn run(args: Cli) -> MyResult<()> {
    let type_filter = |entry: &DirEntry| {
        args.entry_types.is_empty()
            || args.entry_types.iter().any(|entry_type| match entry_type {
                EntryType::Link => entry.path_is_symlink(),
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
            })
    };
    let name_filter = |entry: &DirEntry| {
        args.names.is_empty()
            || args
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in args.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        println!("{}", entries.join("\n"));
    }
    Ok(())
}
